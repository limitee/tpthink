var Com = function(config) {
    var self = this;
    self.config = config;
    self.parent = self.config.parent;
    //子页面
    self.cr = {
    };
    //console.log(self.config.add);
    self.order_id = self.config.add.order_id;
    self.init();
}

Com.prototype.init = function() {
    var self = this;
    self.cur_group_id = -100;
    self.dom_nav_bar = self.parent.find("#nav_bar");
    self.dom_food_info = self.parent.find("#food_info");
    self.selected_food = {};
    self.cache_name = {};
    self.cache_des = {};

    var cond = {};
    var sort = [{id:-1}];
    var body = {
        cond: JSON.stringify(cond),
        sort: JSON.stringify(sort),
        offset: -1,
        limit: -1
    };
    CurSite.postDigest({cmd:"HF02"}, body, function(err, back_body)
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
    }
    html += '<li role="presentation" group_id="-1"><a href="#">本次点餐</a></li>'
    html += '<li role="presentation" group_id="-2"><a href="#">我的订单</a></li>'
    return html;
}

Com.prototype.active_group = function(group_id) {
    var self = this;
    if(self.cur_group_id < -10)
    {
        return;
    }
    self.dom_nav_bar.find('li[group_id="' + self.cur_group_id + '"]').removeClass("active");

    var target = self.dom_nav_bar.find('li[group_id="' + group_id + '"]');
    target.addClass("active");
    self.cur_group_id = group_id;

    self.get_group_food();
}

Com.prototype.get_group_food = function() {
    var self = this;
    if(parseInt(self.cur_group_id) == -1) {
        var data = [];
        for (var key in self.selected_food) {
            var set = self.selected_food[key];
            set.name = self.cache_name[set.id];
            set.des = self.cache_des[set.id];
            if (set.num > 0) {
                data.push(set)
            }
        }
        var html = self.get_list_food_html(data);
        self.dom_food_info.html(html);
        self.init_list_event();
    } else if(parseInt(self.cur_group_id) == -2) {
        var body = {order_id: self.order_id}
        CurSite.postDigest({cmd:"HO03"}, body, function(err, back_body)
        {
            if(back_body) {
                var order_food = back_body.order_food.data;
                var order = back_body.order.data[0]
                var html = self.get_order_food_html(order_food, order);
                self.dom_food_info.html(html);
                self.init_order_list_event();
            }
        });
    } else {
        var cond = {group_id:parseInt(self.cur_group_id)};
        var sort = [{id:1}];
        var body = {
            cond: JSON.stringify(cond),
            sort: JSON.stringify(sort),
            offset: -1,
            limit: -1
        };
        CurSite.postDigest({cmd:"HF05"}, body, function(err, back_body)
        {
            if(back_body) {
                var html = self.get_group_food_html(back_body.data);
                self.dom_food_info.html(html);
                self.init_food_event();
            }
        });
    }
}

Com.prototype.plus_food = function(food) {
    var self = this;
    var target = self.selected_food[food.id];
    if(target)
    {
        target.num++;
    }
    else
    {
        self.selected_food[food.id] = food;
        target = food;
    }
    return target;
}

Com.prototype.minus_food = function(food) {
    var self = this;
    var target = self.selected_food[food.id];
    if(target)
    {
        target.num--;
        if(target.num == 0) {
            delete self.selected_food[food.id]
        }
    }
    return target;
}

Com.prototype.init_list_event = function() {
    var self = this;
    self.dom_food_info.find('[flag="minus"]').on("click", function(e) {
        var id = parseInt($(this).attr("food_id"));
        var food = {
            id:id,
            num:1
        }
        self.minus_food(food);
        self.get_group_food();
    });

    self.dom_food_info.find('[flag="plus"]').on("click", function(e) {
        var id = parseInt($(this).attr("food_id"));
        var food = {
            id:id,
            num:1
        }
        self.plus_food(food);
        self.get_group_food();
    });

    self.dom_food_info.find("#upload").on("click", function(e){
        var btn = $(this).button('loading');
        btn.addClass("disabled");
        var body = {
            order_id: self.order_id,
            data: self.get_upload_data()
        };
        CurSite.postDigest({cmd:"HO02"}, body, function(err, back_body)
        {
            if(back_body) {
                btn.button("end");
                self.selected_food = [];
                //self.dom_food_info.html("提交成功");
            }
        });
    });
}

Com.prototype.init_order_list_event = function() {
    var self = this;

    self.dom_food_info.find("#finish").on("click", function(e){
        //var btn = $(this).button('loading');
        var btn = $(this);
        var body = {
            order_id: self.order_id
        };
        CurSite.postDigest({cmd:"HO04"}, body, function(err, back_body)
        {
            if(back_body) {
                btn.button("loading");
            }
        });
    });
}

Com.prototype.get_upload_data = function() {
    var self = this;
    var data = [];
    for(var key in self.selected_food) {
        var set = self.selected_food[key];
        if(set.num > 0) {
            var new_set = {
                food_id: set.id,
                num: set.num,
                amount: set.price*set.num,
                name: set.name,
                price: set.price,
            }
            data.push(new_set);
        }
    }
    return data;
}

Com.prototype.init_food_event = function() {
    var self = this;
    self.dom_food_info.find('[flag="minus"]').on("click", function(e) {
        var id = parseInt($(this).attr("food_id"));
        var price = parseInt($(this).attr("price"));
        var file_id = parseInt($(this).attr("file_id"));
        var food = {
            id:id,
            num:1,
            price: price,
            file_id:file_id
        }
        var back = self.minus_food(food);
        if(back) {
            $(this).parent().find('[flag="num"]').html(back.num);
            $(this).parent().parent().find('[flag="total"]').html(back.num*back.price/100);
        }
    });

    self.dom_food_info.find('[flag="plus"]').on("click", function(e) {
        var id = parseInt($(this).attr("food_id"));
        var price = parseInt($(this).attr("price"));
        var file_id = parseInt($(this).attr("file_id"));
        var food = {
            id:id,
            num:1,
            price: price,
            file_id: file_id
        }
        var back = self.plus_food(food);
        $(this).parent().find('[flag="num"]').html(back.num);
        $(this).parent().parent().find('[flag="total"]').html(back.num*back.price/100);
    });
}

Com.prototype.get_select_num = function(id) {
    var self = this;
    var target = self.selected_food[id];
    if(target) {
        return target.num;
    }
    return 0;
}

Com.prototype.get_list_food_html = function(sets) {
    var self = this;
    var total = 0;
    var html = '<table class="table table-striped table-hover">';
    html += '<thead><tr>'
    html += '<td>名称</td><td>份数</td><td>价格(元)</td><td>操作<td>';
    html += '</tr></thead><tbody>';
    for(var i = 0; i < sets.length; i++) {
        var set = sets[i];
        var seleted_num = self.get_select_num(set.id);
        var set_price = seleted_num*set.price/100;
        total += seleted_num*set.price;
        html += '<tr><td>' + set.name + '</td>'
            + '<td>' + seleted_num + '</td>'
            + '<td>' + set_price + '</td>'
            + '<td>'
            + '    <button flag="minus" food_id="' + set.id + '" type="button" class="btn btn-default"><span class="glyphicon glyphicon-minus" aria-hidden="true"></span></button>'
            + '    <button flag="plus" food_id="' + set.id + '" type="button" class="btn btn-default"><span class="glyphicon glyphicon-plus" aria-hidden="true"></span></button>'
            + '</td>'
            + '</tr>';
    }
    html += '<tr><td colspan="2">总计</td><td colspan="2">' + total/100 + '</td></tr>';
    html += '</tbody>';
    html += '</table>';
    if(total > 0) {
        html += '<button id="upload" type="button" data-loading-text="处理中.." data-end-text="已成功提交" class="btn-lg btn-default">确定</button>';
    }
    return html;
}

Com.prototype.get_order_food_html = function(sets, order) {
    var self = this;
    var total = 0;
    var html = '<table class="table table-striped table-hover">';
    html += '<thead><tr>'
    html += '<td>名称</td><td>份数</td><td>价格(元)</td>';
    html += '</tr></thead><tbody>';
    for(var i = 0; i < sets.length; i++) {
        var set = sets[i];
        var seleted_num = set.num;
        var set_price = seleted_num*set.price/100;
        total += seleted_num*set.price;
        html += '<tr><td>' + set.name + '</td>'
            + '<td>' + seleted_num + '</td>'
            + '<td>' + set_price + '</td>'
            + '</tr>';
    }
    html += '<tr><td colspan="2">总计</td><td colspan="1">' + total/100 + '</td></tr>';
    html += '</tbody>';
    html += '</table>';
    if(order.status == 0) {
        html += '<button id="finish" type="button" data-loading-text="已成功提交" class="btn-lg btn-default">结账</button>';
    } else if(order.status == 100) {
        html += '<button type="button" class="btn-lg btn-default disabled">已结账</button>';
    }
    return html;
}

Com.prototype.get_group_food_html = function(sets) {
    var self = this;
    var html = "";
    html += '<div class="row">';
    for(var i = 0; i < sets.length; i++)
    {
        var set = sets[i];
        self.cache_name[set.id] = set.name;
        self.cache_des[set.id] = set.des;
        if(i%2 == 0 && i > 0)
        {
            html += '</div><div class="row">';
        }
        var seleted_num = self.get_select_num(set.id);
        html +=
            '<div class="col-md-6">'
            + '<div class="thumbnail">'
            + '    <div class="caption">'
            + '        <div class="row"><div class="col-md-12">' + set.name + '</div></div>'
            + '        <div class="row">'
            + '             <div class="col-md-6">'
            + '                 <img width="100%" src="./api/file/' + set.file_id + '" class="img-rounded" style="border:1px solid;"/>'
            + '             </div>'
            + '             <div class="col-md-6">' + set.des
            + '             </div>'
            + '        </div>'
            + '        <div class="row"><div class="col-md-8">'
            + '        <button flag="minus" file_id="' + set.file_id + '" price="' + set.price + '" food_id="' + set.id + '" type="button" class="btn btn-default"><span class="glyphicon glyphicon-minus" aria-hidden="true"></span></button>'
            + '        <span flag="num">' + seleted_num + '</span>(份)*' + set.price/100.0 + '(元)'
            + '        <button flag="plus" file_id="' + set.file_id + '" price="' + set.price + '" food_id="' + set.id + '" type="button" class="btn btn-default"><span class="glyphicon glyphicon-plus" aria-hidden="true"></span></button>'
            + '        </div><div class="col-md-4 text-info">'
            + '        <span flag="total">' + seleted_num*set.price/100 + '</span>(元)'
            + '        </div></div>'
            + '    </div>'
            + '</div>'
            + '</div>';
    }
    html += '</div>';
    return html;
}

return Com;

