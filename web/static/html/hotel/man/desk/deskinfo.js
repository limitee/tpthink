var Com = function(config) {
    var self = this;
    self.config = config;
    self.parent = self.config.parent;
    //子页面
    self.cr = {
    };
    self.desk_id = self.config.add.desk_id;
    self.order_id = -1;
    self.init();
}

Com.prototype.init = function() {
    var self = this;
    self.dom_handle_entry = self.parent.find("#handle_entry");

    var cond = {id:self.desk_id};
    var sort = [];
    var body = {
        cond: JSON.stringify(cond),
        sort: JSON.stringify(sort),
        offset: -1,
        limit: -1
    };
    CurSite.postDigest({cmd:"HD05"}, body, function(err, back_body)
    {
        if(back_body) {
            var set = back_body.data[0];
            if(set.status == 100)   //就餐中,设置order_id
            {
                self.order_id = set.cur_order_id;
            }
            self.dom_handle_entry.html(self.get_html(set));
            self.init_event();
        }
    });
}

Com.prototype.get_html = function(set) {
    var self = this;
    var html = "";
    //console.log(set);
    var sets = [];
    if(set.status == 0)
    {
        sets.push({"name": "点餐", type:1});
    }
    else if(set.status == 100)
    {
        sets.push({"name": "点餐", type:2});
        sets.push({"name": "结账", type:3});
    }
    for(var i = 0; i < sets.length; i++)
    {
        html += self.get_set_html(sets[i]);
    }
    return html;
}

Com.prototype.init_event = function() {
    var self = this;
    self.dom_handle_entry.find("a").on("click", function(e) {
        var type = parseInt($(this).attr("type"));
        if(type == 1)
        {
            //创建订单
            var body = {desk_id:self.desk_id};
            CurSite.postDigest({cmd:"HO01"}, body, function(err, back_body)
            {
                if(back_body) {
                    self.order_id = back_body.order_id;
                    self.select_food();
                }
            });
        }
        else if(type == 2) {
            self.select_food();
        }
    });
}

Com.prototype.select_food = function() {
    var self = this;
    var add = {
        "desk_id": self.desk_id,
        "order_id": self.order_id
    }
    var page = {
        "url": "hotel_man_food_selectfood",
        "name": "点餐",
        add: add
    }
    self.config.pins.append(page);
}

Com.prototype.get_set_html = function(set) {
    var self = this;
    var html = '';
    html +=
        '<div class="col-md-6">'
        + '<div class="thumbnail">'
        + '    <div class="caption">'
        + '        <div class="row">'
        + '             <div class="col-md-6">' + set.name
        + '             </div>'
        + '             <div class="col-md-6 text-right">'
        + '                 <a style="cursor: pointer" type="' + set.type + '">进入>></a>'
        + '             </div>'
        + '        </div>'
        + '    </div>'
        + '</div>'
        + '</div>';
    return html;
}

return Com;

