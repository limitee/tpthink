var TreeNode = function() {
    var self = this;
    self.type = 0;  //-1,mkd text, 0,plain text not parsed, 1,text url has parsed, 2,unit
    self.text = '';
};

TreeNode.inst = function(text, type) {
    var node = new TreeNode();
    node.text = text;
    node.type = type;
    return node;
};

TreeNode.prototype.parse = function() {
    var self = this; 
    if(self.type != 0) return;
    var node_list = TreeNode.parse_url(self.text);
    node_list = TreeNode.parse_code(node_list);
    node_list = TreeNode.parse_strong(node_list);
    node_list = TreeNode.parse_em(node_list);
    return node_list;
};

/**
 * get the char_list, replace the pos, get the flag_list
 */
TreeNode.get_char_list = function(node_list, flag_list, pos_list) {
    var char_list = '';
    for(var i = 0; i < node_list.length; i++) {
        var node = node_list[i];
        if(node.type == 0) {
            char_list += node.text;
        }
        if(node.type == 1) {  //before the flag pos, has a html
            flag_list[char_list.length] = i;
            pos_list.push(char_list.length);
            char_list += '_';
        }
    }
    return char_list;
};

TreeNode.get_node_list = function(node_list, char_list, flag_list, pos_list, seg_list, flag) {
    var cur_seg_index = 0;
    //find the flags's owner.
    for(var i = 0; i < pos_list.length; i++) {
        var pos = pos_list[i];
        for(var j = 0; j < seg_list.length; j++) {
            var seg = seg_list[j];
            if(pos >= seg.start && pos <= seg.end) {
                seg.flags.push(pos);
            }
        }
    }
    var new_node_list = [];
    //generate the node_list
    for(var i = 0; i < seg_list.length; i++) {
        var seg = seg_list[i]; 
        if(seg.type == 0) {
            if(seg.flags.length == 0) {
                var node = TreeNode.inst(char_list.substr(seg.start, seg.end - seg.start + 1), 0);
                new_node_list.push(node);
            } else {
                var start = seg.start;
                for(var j = 0; j < seg.flags.length; j++) {
                    var pos = seg.flags[j];
                    if(pos > start) {  //start plain text
                        var node = TreeNode.inst(char_list.substr(start, pos - start), 0);
                        new_node_list.push(node);
                    }
                    var node = node_list[flag_list[pos]];
                    new_node_list.push(node);
                    start = pos + 1;
                }
                if(start <= seg.end) {
                    var node = TreeNode.inst(char_list.substr(start, seg.end - start + 1), 0);
                    new_node_list.push(node);
                }
            }
        }

        if(seg.type == 1) {
            if(seg.flags.length == 0) {
                var tmp = char_list.substr(seg.start, seg.end - seg.start + 1); 
                tmp = MkDown.escape_html(tmp);
                var html = '<' + flag + '>' + tmp + '</' + flag + '>';
                var node = TreeNode.inst(html, 1);
                new_node_list.push(node);
            } else {
                var html = '';
                var start = seg.start;
                for(var j = 0; j < seg.flags.length; j++) {
                    var pos = seg.flags[j];
                    if(pos > start) {  //start plain text
                        var tmp = char_list.substr(start, pos - start);
                        tmp = MkDown.escape_html(tmp);
                        html += tmp;
                    }
                    var node = node_list[flag_list[pos]];
                    html += node.text;
                    start = pos + 1;
                }
                if(start <= seg.end) {
                    var tmp = char_list.substr(start, seg.end - start + 1);
                    tmp = MkDown.escape_html(tmp);
                    html += tmp;
                }
                var html = '<'+ flag + '>' + html + '</' + flag + '>';
                var node = TreeNode.inst(html, 1);
                new_node_list.push(node);
            }
        }
    }
    return new_node_list;
};

/* * and *  means em flag */
TreeNode.parse_em = function(node_list) {
    var flag_list = {};
    var pos_list = [];
    var char_list = TreeNode.get_char_list(node_list, flag_list, pos_list);
    var seg_list = [];
    var in_code = false;
    var start = 0;
    for(var i = 0; i < char_list.length; i++) {
        var c = char_list.charAt(i);
        if(c == '*') {
            if(!in_code) {
                seg_list.push({start:start, end:i - 1, type:0, flags:[]}); 
                in_code = true;
            } else {
                end = i - 1;
                seg_list.push({start:start, end:end, type:1, flags:[]});
                in_code = false;
            }
            start = i + 1;
        }
    }
    if(start <= char_list.length - 1) { //get the end part
        if(in_code) {
            start = start - 1;
        }
        seg_list.push({start:start, end:char_list.length - 1, type:0, flags:[]});
    }
    var new_node_list = TreeNode.get_node_list(node_list, char_list, flag_list, pos_list, seg_list, 'em');
    return new_node_list;
};

/* ` and `  means code flag */
TreeNode.parse_code = function(node_list) {
    var flag_list = {};
    var pos_list = [];
    var char_list = TreeNode.get_char_list(node_list, flag_list, pos_list);
    var seg_list = [];
    var in_code = false;
    var start = 0;
    for(var i = 0; i < char_list.length; i++) {
        var c = char_list.charAt(i);
        if(c == '`') {
            if(!in_code) {
                seg_list.push({start:start, end:i - 1, type:0, flags:[]}); 
                in_code = true;
            } else {
                end = i - 1;
                seg_list.push({start:start, end:end, type:1, flags:[]});
                in_code = false;
            }
            start = i + 1;
        }
    }
    if(start <= char_list.length - 1) { //get the end part
        if(in_code) {
            start = start - 1;
        }
        seg_list.push({start:start, end:char_list.length - 1, type:0, flags:[]});
    }
    var new_node_list = TreeNode.get_node_list(node_list, char_list, flag_list, pos_list, seg_list, 'code');
    return new_node_list;
};

/* ** and ** means strong flag */
TreeNode.parse_strong = function(node_list) {
    var flag_list = {};
    var pos_list = [];
    var char_list = TreeNode.get_char_list(node_list, flag_list, pos_list);
    var seg_list = [];
    var in_strong = false;
    var start = 0;
    for(var i = 0; i < char_list.length; i++) {
        var c = char_list.charAt(i);
        if(i + 1 < char_list.length) {
            var next_c = char_list.charAt(i + 1); 
        }
        else
        {
            var next_c = null;
        }
        if(c == '*' && next_c == '*') {
            if(!in_strong) {
                seg_list.push({start:start, end:i - 1, type:0, flags:[]}); 
                in_strong = true;
                start = i + 2;
            } else {
                end = i - 1;
                seg_list.push({start:start, end:end, type:1, flags:[]});
                in_strong = false;
                start = i + 2;
            }
            i++;
        }
    }
    if(start <= char_list.length - 1) { //get the end part
        if(in_strong) {
            start = start - 2;
        }
        seg_list.push({start:start, end:char_list.length - 1, type:0, flags:[]});
    }
    var node_list = TreeNode.parse_code(node_list);
    var new_node_list = TreeNode.get_node_list(node_list, char_list, flag_list, pos_list, seg_list, 'strong');
    return new_node_list;
};

TreeNode.parse_url = function(content) {
    var self = this;
    var stack = [')', '(', ']', '['];
    var pos = [];   //store the pos of the flag
    var node_list = [];
    var last_node_index = 0;
    var last_char = null;
    for(var i = 0; i < content.length; i++) {
        var c = content.charAt(i);
        var expect = stack[stack.length - 1];
        if(c == expect) {
            if(stack.length == 4 && i > 0) {
                last_char = content.charAt(i - 1);
            }
            pos.push(i);
            stack.pop();
            if(stack.length == 0) {
                var title = content.substr(pos[0] + 1, pos[1] - pos[0] - 1);
                var url = content.substr(pos[2] + 1, pos[3] - pos[2] - 1);
                var last_node_len = pos[0] - last_node_index;
                if(last_char == '!') { //if it's a pic
                    last_node_len--;
                }
                if(last_node_len > 0) {
                    var last_node = TreeNode.inst(content.substr(last_node_index, last_node_len), 0);
                    node_list.push(last_node);
                }
                if(last_char == '!') {
                    var node = TreeNode.inst('<img src="' + url + '"/>', 1);
                } else {
                    var node = TreeNode.inst('<a href="' + url + '">' + title + '</a>', 1);
                }
                node_list.push(node);

                stack = [')', '(', ']', '['];
                last_node_index = pos[3] + 1;
                pos = [];
            }
        }
    }
    if(last_node_index < content.length) {
        var last_node = TreeNode.inst(content.substr(last_node_index), 0);
        node_list.push(last_node);
    }
    return node_list;
};

TreeNode.prototype.to_html = function() {
    var self = this;
    var node_list = self.parse();
    var rst = '';
    for(var i = 0; i < node_list.length; i++) {
        var node = node_list[i];
        if(node.type == 0) {
            rst += MkDown.escape_html(node.text);
        }
        else
        {
            rst += node.text; 
        }
    }
    return rst;
};

var MkDown = function() {};

MkDown.escape_html = function(html) {
      return html.replace(/&/g,'&amp;').replace(/>/g,'&gt;').replace(/</g,'&lt;').replace(/"/g,'&quot;');

};

MkDown.start_with = function(str, sub) {
    if(str.length < sub.length)
    {
        return false;
    }
    for(var i = 0; i < str.length; i++) {
        var c_f = str.charAt(i);
        var c_t = sub.charAt(i);
        if(c_f != c_t) {
            return false;
        }
        if(i == sub.length - 1) {
            return true;
        }
    }
    return false;
};

MkDown.prototype.to_html = function(content) {
	var self = this;
	var rst = '';
    var in_pre_code = false;
    var in_ul = false;
    var in_ol = false;
    var pre_content = '';
    var ul_content = '';
    var ol_content = '';
    var ol_index = 1;
	var lines = content.split("\n");
	for(var i = 0; i < lines.length; i++)
	{
        var line = lines[i];
        if(!in_pre_code && !in_ul && !in_ol) {
            if(MkDown.start_with(line, '```')) {
                in_pre_code = true;
            } else if(MkDown.start_with(line, '- ') || MkDown.start_with(line, '* ')) {
                in_ul = true;
                var node = TreeNode.inst(line.substr(2), 0);
                ul_content += '<li>' + node.to_html() + '</li>';
                console.log(ul_content);
                if(i >= lines.length - 1 || !(MkDown.start_with(lines[i + 1], '- ') || MkDown.start_with(lines[i + 1], '* '))) {
                    in_ul = false;
                    rst += '<ul>' + ul_content + '</ul>';
                    ul_content = '';
                }
            } else if(MkDown.start_with(line, ol_index + '.')) {
                in_ol = true;
                var node = TreeNode.inst(line.substr(2), 0);
                ol_content += '<li>' + node.to_html() + '</li>';
                ol_index++;
                if(i >= lines.length - 1 || !MkDown.start_with(lines[i + 1], ol_index + '.')) {
                    in_ol = false;
                    rst += '<ol>' + ol_content + '</ol>';
                    ol_content = '';
                }
            } else {
		        rst += self.parse_line(line);
            }
        } else {
            if(in_pre_code) {
                if(MkDown.start_with(line, '```')) {
                    in_pre_code = false;
                    rst += '<pre><code>' + MkDown.escape_html(pre_content) + '</code></pre>';
                    pre_content = '';
                } else {
                    pre_content += line + '\n';  
                }
            }
            
            if(in_ul) {
                var node = TreeNode.inst(line.substr(2), 0);
                ul_content += '<li>' + node.to_html() + '</li>';
                if(i >= lines.length - 1 || !(MkDown.start_with(lines[i + 1], '- ') || MkDown.start_with(lines[i + 1], '* '))) {
                    in_ul = false;
                    rst += '<ul>' + ul_content + '</ul>';
                    ul_content = '';
                }
            }

            if(in_ol) {
                var node = TreeNode.inst(line.substr(2), 0);
                ol_content += '<li>' + node.to_html() + '</li>';
                ol_index++;
                if(i >= lines.length - 1 || !MkDown.start_with(lines[i + 1], ol_index + '.')) {
                    in_ol = false;
                    rst += '<ol>' + ol_content + '</ol>';
                    ol_content = '';
                }
            }
        }
	}
	return rst;
};

MkDown.prototype.parse_line = function(line) {
	var self = this;
	var start = line.charAt(0);
    var rst = '';
	switch(start) {
		case '>':
            rst = self.parse_quote(line);
			break;
        case '#':
            rst = self.parse_head(line);
            break;
        case '*':
            rst = self.parse_hr(line);
            break;
        case '-':
            rst = self.parse_hr(line);
            break;
		default :
            var node = TreeNode.inst(line, 0);
            rst = '<p>' + node.to_html() + '</p>';
	}
    return rst; 
};

MkDown.prototype.parse_quote = function(line) {
    var self = this; 
    var content = line.substr(1);
    var node = TreeNode.inst(content, 0);
    return '<blockquote>' + node.to_html() + '</blockquote>';
};

MkDown.prototype.parse_head = function(line) {
    var self = this;
    var h_count = 0;
    for(var i = 0; i < line.length; i++) {
        var c = line.charAt(i);
        if(c == '#') {
            h_count++;
        } else {
            break;
        }
    }
    var content = line.substr(h_count);
    var node = TreeNode.inst(content, 0);
    var flag = 'h' + (h_count + 1);
    return '<' + flag + '>' + node.to_html() + '</' + flag + '>';
};

MkDown.prototype.parse_hr = function(line) {
    var self = this;
    var hr_count = 0;
    for(var i = 0; i < line.length; i++) {
        var c = line.charAt(i);
        if(c == '*' || c == '-') {
            hr_count++; 
        } else {
            break; 
        }
    }
    if(hr_count == line.length) {
        return '<hr/>';
    } else {
        var node = TreeNode.inst(line, 0);
        return '<p>' + node.to_html() + '</p>';
    }
};

var mk = new MkDown();
//var node = TreeNode.inst('![abc](http://www.test.com)j', 0);
//console.log(node.to_html());
//var html = mk.to_html(">this is a **`quote`**.`**`[just](http://www.test.com)**, **nothing.[justi2](http://www.test2.com), ![justi2](http://www.test2.com/test.png)");
//console.log(html);




