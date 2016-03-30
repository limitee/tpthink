var Com = function(config) {
	var self = this;
	self.config = config;
    self.parent = self.config.parent;
    //子页面
    self.cr = {
        'pagebar': {
            pins: self,
            parent: self.parent.children('#pagebar')
        }
    };
    self.skip = 0;
    self.limit = 10;
    self.sort = [{id:-1}];
    self.cond = {};
	self.init();
};

Com.prototype.init = function() {
	var self = this;
    self.dom_set_list = self.parent.find('#user_list');
    self.to_page(1);
};

Com.prototype.to_page = function(index) {
    var self = this;
    self.skip = (index - 1)*self.limit;
    var cond = JSON.stringify(self.cond);
    var sort = JSON.stringify(self.sort);
    var body = {
        cond: cond,
        sort: sort,
        offset: self.skip,
        limit: self.limit
    };
    CurSite.postDigest({cmd:"F01"}, body, function(err, back_body)
    {
        if(back_body.data.length == 0) {
            self.dom_set_list.html("列表为空");
            return;
        }
        self.dom_set_list.html(self.get_table(back_body.data));
        self.cr.pagebar.add = {
            skip: self.skip,
            limit: self.limit,
            total: back_body.count
        }
        CurSite.to_page(self.cr.pagebar, "sys_pagebar");
        self.init_event();
    });
}

Com.prototype.init_event = function() {
    var self = this;
}

Com.prototype.get_selected_id = function() {
    var self = this;
    var input_dom = self.parent.find('input[name="select_img"]:checked');
    if(input_dom.length == 0)
    {
        return -1;
    }
    else
    {
        var id = input_dom.val();
        return parseInt(id)
    }
}

Com.prototype.get_table = function(data) {
    var self = this;
    var html = '<table class="table table-striped table-bordered table-hover">';
    html += '<thead><tr><td width="120px">操作</td><td width="120px">名称</td><td>图片</td></tr></thead>';
    html += '<tbody>';
    for(var i = 0; i < data.length; i++) {
        html += '<tr>'
        html += '<td><input name="select_img" type="radio" value="' + data[i].id + '"/></td>'
        html += '<td>' + data[i].name + '</td>'
        html += '<td>'
        html += '<img width="120px" src="./api/file/' + data[i].id + '"/>';
        html += '</td>'
        html += '</tr>'
    }
    html += '</tbody>';
    html += '</table>';
    return html;
}

return Com;
