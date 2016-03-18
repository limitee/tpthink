var Com = function(config) {
    var self = this;
    self.config = config;
    self.parent = self.config.parent;
    //子页面
    self.cr = {
    };
    self.init();
}

Com.prototype.init = function() {
    var self = this;
    var add = self.config.add;
    self.cur = add.skip/add.limit + 1; //current page
    self.page_count = parseInt(add.total/add.limit);
    if(add.total%add.limit > 0) {
        self.page_count++;
    }
    var index_array = self.get_index_array();
    var html = self.get_html(index_array);
    self.parent.html(html);
    self.handle_event();
}

Com.prototype.handle_event = function() {
    var self = this;
    var a_objs = self.parent.find("a");
    a_objs.on("click", function(e) {
        var p_index = $(this).attr("pIndex");        
        if(p_index > 0 && p_index != self.cur && p_index <= self.page_count) {
            if(self.config.pins.to_page) {
                self.config.pins.to_page(p_index); 
            }
        }
        e.preventDefault();
    });
}

Com.prototype.get_html = function(index_array) {
    var self = this;
    var html = '<nav><ul class="pagination">';
    if(self.cur == 1) {
        var cl = 'disable';
    } else {
        var cl = '';
    }
    var last = '<li class="' + cl + '"><a pIndex="' + (self.cur - 1) + '" aria-label="Previous">';
    last +=        '<span aria-hidden="true">';
    last +=        '&laquo;';
    last +=        '</span>';
    last +=    '</a></li>';
    html += last;
    for(var i = 0; i < index_array.length; i++) {
        var index = index_array[i];
        if(index > 0) {
            if(index == self.cur) {
                var cl = 'active';
            } else {
                var cl = '';
            }
            var lb = index;
        } else {
            var cl = 'disable';
            var lb = '...';
        }
        var sep = '<li class="' + cl + '">' + 
                       '<a pIndex="' + index + '">' + 
                       lb + 
                       '</a>' + 
                  '</li>';
        html += sep;
    }
    if(self.cur == self.page_count) {
        var cl = 'disable';
    } else {
        var cl = '';
    }
    var next = '<li class="' + cl + '"><a pIndex="' + (self.cur + 1) + '" aria-label="Next">' +
                   '<span aria-hidden="true">' + 
                   '&raquo;' +
                   '</span>' +
               '</a></li>';
    html += next;
    html +=    '</ul></nav>';
    return html;
}

Com.prototype.get_index_array = function() {
    var self = this;
    var add = self.config.add;
    var page_count = self.page_count;
    var array = [];
    if(page_count < 7) {
        for(var i = 0; i < page_count; i++) {
            array.push(i + 1);
        }
    } else {
        if(self.cur < 4 || page_count - self.cur < 3) {
           array.push(1);array.push(2);array.push(3);
           array.push(-1);
           array.push(page_count - 2);array.push(page_count - 1);array.push(page_count);
        } else {
           array.push(1);array.push(-1);
           array.push(self.cur - 1);
           array.push(self.cur);
           array.push(self.cur + 1);
           array.push(-1);array.push(page_count);
        }
    }
    return array;
}

return Com;
