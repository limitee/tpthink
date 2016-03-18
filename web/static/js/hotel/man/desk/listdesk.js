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
}

Com.prototype.init = function() {
    var self = this;
    self.dom_set_list = self.parent.find('#set_list');

    self.group_list = {}
    var cond = JSON.stringify({});
    var sort = JSON.stringify([{id:-1}]);
    var body = {
        cond: cond,
        sort: sort,
        limit: -1,
        offset: -1
    };
    CurSite.postDigest({cmd:"HD02"}, body, function(err, back_body)
    {
        if(back_body) {
            var sets = back_body.data;
            for(var i = 0; i < sets.length; i++) {
                var set = sets[i];
                self.group_list[set.id] = set.name;
            }
            self.to_page(1);
        }
    });
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
    CurSite.postDigest({cmd:"HD05"}, body, function(err, back_body)
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
        var desk_id = $(this).attr("desk_id");
        var body = {
            id:parseInt(desk_id)
        }
        CurSite.postDigest({cmd:"HD06"}, body, function(err, back_body)
        {
            var cur = self.skip/self.limit + 1;
            self.to_page(cur);
        });
    });
}

Com.prototype.get_table = function(data) {
    var self = this;
    var html = '<table class="table table-striped table-hover">';
    html += '<thead><tr><td>名称</td><td>所属分组</td><td>创建时间</td><td>操作<td></tr></thead>';
    html += '<tbody>';
    for(var i = 0; i < data.length; i++) {
        html += '<tr><td>' + data[i].name + '</td>'
        + '<td>' + self.group_list[data[i].group_id] + '</td>'
        + '<td>' + CurSite.getDateStr(data[i].create_time*1000) + '</td>'
        + '<td>'
        + '<a desk_id="' + data[i].id + '" flag="delete">删除</a>'
        + '</td>'
        + '</tr>';
    }
    html += '</tbody>';
    html += '</table>';
    return html;
}

return Com;

