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
    self.sort = [{index:1}];
    self.cond = {};
    self.init();
}

Com.prototype.init = function() {
    var self = this;
    self.dom_set_list = self.parent.find('#room_type_list');
    self.to_page(1);
}

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
    CurSite.postDigest({cmd:"ART01"}, body, function(err, back_body)
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
    self.dom_set_list.find('a[flag="up"]').on("click", function(e) {
        var book_type_id = $(this).attr("type_id");
        var index = $(this).attr("index");
        self.change(book_type_id, index, -1);
    });
    
    self.dom_set_list.find('a[flag="down"]').on("click", function(e) {
        var book_type_id = $(this).attr("type_id");
        var index = $(this).attr("index");
        self.change(book_type_id, index, 1);
    });

    self.dom_set_list.find('a[flag="delete"]').on("click", function(e) {
        var type_id = $(this).attr("type_id");
        var body = {
            id:parseInt(type_id)
        }
        CurSite.postDigest({cmd:"ART04"}, body, function(err, back_body)
        {
            var cur = self.skip/self.limit + 1;
            self.to_page(cur);
        });
    });
}

Com.prototype.change = function(id, index, up_or_down) {
    var self = this;
    var body = {
        id:parseInt(id),
        index:parseInt(index),
        up_or_down: up_or_down
    }
    CurSite.postDigest({cmd:"ART03"}, body, function(err, back_body)
    {
        var cur = self.skip/self.limit + 1;
        self.to_page(cur);
    });
}

Com.prototype.get_table = function(data) {
    var self = this;
    var html = '<table class="table table-striped table-hover">';
    html += '<thead><tr><td>序号</td><td>名称</td><td>价格</td><td>创建时间</td><td>操作<td></tr></thead>';
    html += '<tbody>';
    for(var i = 0; i < data.length; i++) {
        html += '<tr><td>' + data[i].index + '</td><td>' + data[i].name + '</td>'
        + '<td>' + data[i].price + '</td>'
        + '<td>' + CurSite.getDateStr(data[i].create_time*1000) + '</td>'
        + '<td>'
        + '<a type_id="' + data[i].id + '" index="' + data[i].index + '" flag="up">上移</a>&nbsp;&nbsp;' 
        + '<a type_id="' + data[i].id + '" index="' + data[i].index + '" flag="down">下移</a>&nbsp;&nbsp;'
        + '<a type_id="' + data[i].id + '" flag="delete">删除</a>'
        + '</td>'
        + '</tr>';
    }
    html += '</tbody>';
    html += '</table>';
    return html;
}

return Com;

