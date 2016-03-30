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

    self.dom_set_list.find('a[flag="delete"]').on("click", function(e) {
        var id = $(this).attr("file_id");
        var body = {
            id:parseInt(id)
        }
        if(confirm("确实要删除这个文件吗"))
        {
            CurSite.postDigest({cmd:"F04"}, body, function(err, back_body)
            {
                var cur = self.skip/self.limit + 1;
                self.to_page(cur);
            });
        }
    });
}

Com.prototype.get_table = function(data) {
    var self = this;
    var html = '<table class="table table-striped table-bordered table-hover">';
    html += '<thead><tr><td>名称</td><td>类型</td><td>大小</td><td>创建时间</td><td>操作</td></tr></thead>';
    html += '<tbody>';
    for(var i = 0; i < data.length; i++) {
        html += '<tr>'
        html += '<td>' + data[i].name + '</td><td>' + data[i].type + '</td><td>' + data[i].size + '</td><td>' + CurSite.getDateStr(data[i].create_time*1000) + '</td>';
        html += '<td>'
        html += '<a target="_blank" href="http://localhost:4000/api/file/' + data[i].id + '">下载</a>&nbsp;&nbsp;&nbsp;&nbsp;';
        html += '<a file_id="' + data[i].id + '" flag="delete">删除</a>';
        html += '</td>'
        html += '</tr>'
    }
    html += '</tbody>';
    html += '</table>';
    return html;
}

return Com;
