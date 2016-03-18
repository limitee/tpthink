var Com = function(config) {
    var self = this;
    self.config = config;
    self.parent = self.config.parent;
    //子页面
    self.cr = {
    };
    self.skip = 0;
    self.limit = 10;
    self.sort = [{id:-1}];
    self.cond = {};
    self.init();

    self.group_relation = {};
    self.desk_relation = {};
}

Com.prototype.init = function() {
    var self = this;
    self.cur_group_id = -1;
    self.dom_nav_bar = self.parent.find("#nav_bar");
    self.dom_desk_info = self.parent.find("#desk_info");

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
            var html = self.get_group_html(sets);
            self.dom_nav_bar.html(html);

            self.active_group(self.cur_group_id);

            self.dom_nav_bar.find("li").on("click", function(e) {
                var group_id = $(this).attr("group_id");
                self.active_group(group_id);
            });
        }
    });
}

Com.prototype.get_group_desk = function() {
    var self = this;

    var cond = {group_id:parseInt(self.cur_group_id)};
    var sort = [{id:1}];
    var body = {
        cond: JSON.stringify(cond),
        sort: JSON.stringify(sort),
        offset: -1,
        limit: -1
    };
    CurSite.postDigest({cmd:"HD05"}, body, function(err, back_body)
    {
        if(back_body) {
            var html = self.get_group_desk_html(back_body.data);
            self.dom_desk_info.html(html);
            self.init_desk_event();
        }
    });
}

Com.prototype.init_desk_event = function() {
    var self = this;
    self.dom_desk_info.find("a").on("click", function(e) {
        var desk_id = parseInt($(this).attr("desk_id"));

        var group_name = self.group_relation[self.cur_group_id];
        var desk_name = self.desk_relation[desk_id];
        var add = {
            "desk_id": desk_id,
            "desk_name": desk_name,
            "group_name": group_name
        }
        var page = {
            "url": "hotel_man_desk_deskinfo",
            "name": group_name + "第" + desk_name + "桌",
            add: add
        }
        self.config.pins.append(page);
    });
}

Com.prototype.get_group_desk_html = function(sets) {
    var self = this;
    var html = "";
    html += '<div class="row">';
    for(var i = 0; i < sets.length; i++)
    {
        var set = sets[i];
        if(i%2 == 0 && i > 0)
        {
            html += '</div><div class="row">';
        }
        html +=
            '<div class="col-md-6">'
            + '<div class="thumbnail">'
            + '    <div class="caption">'
            + '        <div class="row">'
            + '             <div class="col-md-6">第' + set.name + '桌('
            +               self.get_status_des(set.status) + ')'
            + '             </div>'
            + '             <div class="col-md-6 text-right">'
            + '                 <a desk_id="' + set.id + '" style="cursor: pointer">进入>></a>'
            + '             </div>'
            + '        </div>'
            + '    </div>'
            + '</div>'
            + '</div>';
        self.desk_relation[set.id] = set.name;
    }
    html += '</div>';
    return html;
}

Com.prototype.get_status_des = function(status) {
    var des;
    switch(status) {
        case 0:
            des = "空闲";
            break;
        case 100:
            des = "就餐中";
            break;
        default:
            des = "未知";
    }
    return des;
}

Com.prototype.active_group = function(group_id) {
    var self = this;
    if(self.cur_group_id < 0)
    {
        return;
    }
    self.dom_nav_bar.find('li[group_id="' + self.cur_group_id + '"]').removeClass("active");

    var target = self.dom_nav_bar.find('li[group_id="' + group_id + '"]');
    target.addClass("active");
    self.cur_group_id = group_id;

    self.get_group_desk();
}

Com.prototype.get_group_html = function(sets) {
    var self = this;
    //console.log(sets);
    var html = "";
    for(var i = 0; i < sets.length; i++)
    {
        var set = sets[i];
        if(i == 0) {
            self.cur_group_id = set.id;
        }
        html += '<li role="presentation" group_id="'
            + set.id + '"><a href="#">'
            + set.name
            + '</a></li>';

        self.group_relation[set.id] = set.name;
    }
    return html;
}

return Com;

