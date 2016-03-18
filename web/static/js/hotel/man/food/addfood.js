var Com = function(config) {
    var self = this;
    self.config = config;
    self.parent = self.config.parent;

    //子页面
    self.cr = {
        'select_img': {
            pins: self,
            parent: self.parent.find('#myModalBody')
        }
    };

    self.file_id = -1;
    self.init();
};

Com.prototype.init = function() {
    var self = this;

    self.dom_modal = self.parent.find("#myModal");
    self.dom_modal.on('shown.bs.modal', function (e) {
        CurSite.to_page(self.cr.select_img, "hotel_man_file_list");
    });

    self.dom_file_id = self.parent.find("#file_id");
    self.dom_des = self.parent.find("#des");
    self.dom_price = self.parent.find("#price");

    self.dom_select_btn = self.parent.find("#select");
    self.dom_select_btn.on('click', function(e){
        var id = self.cr.select_img.ins.get_selected_id();
        if(id > 0)
        {
            self.dom_modal.modal('hide');

            self.file_id = id;
            self.dom_file_id.attr("src", "./api/file/" + id)
        }
        else
        {
            alert("请选择一张图片");
        }
    });

    self.dom_sub = self.parent.find("#sub");
    self.dom_name = self.parent.find("#name");
    self.dom_sub.on("click", function(e) {
        self.dom_sub.button("loading");
        var data = self.get_data();
        if(self.check(data))
        {
            self.save(data);
        }
        else
        {
            self.dom_sub.button("reset");
        }
    });

    var cond = JSON.stringify({});
    var sort = JSON.stringify([{id:-1}]);
    var body = {
        cond: cond,
        sort: sort,
        limit: -1,
        offset: -1
    };

    self.dom_group_id = self.parent.find("#group_id");
    CurSite.postDigest({cmd:"HF02"}, body, function(err, back_body)
    {
        if(back_body) {
            var html = '';
            var sets = back_body.data;
            for(var i = 0; i < sets.length; i++) {
                var set = sets[i];
                html += '<option value=' + set.id + '>' + set.name + '</option>';
            }
            self.dom_group_id.html(html);
        }
    });
};

Com.prototype.save = function(data) {
    var self = this;
    var body = data;
    CurSite.postDigest({cmd:"HF04"}, body, function(err, back_body)
    {
        self.dom_sub.button("reset");
        self.parent.html("操作成功")
    });
}

Com.prototype.check = function(data) {
    if(data.name.length === 0)
    {
        alert("必须填名称");
        return false;
    }
    if(data.price.length == 0) {
        alert("请填写菜品的价格");
        return false;
    }
    if(data.file_id < 0)
    {
        alert("请选择一张图片");
        return false;
    }
    data.price = parseInt(parseFloat(data.price)*100);
    return true;
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.name = self.dom_name.val();
    data.price = self.dom_price.val();
    data.group_id = parseInt(self.dom_group_id.val());
    data.file_id = self.file_id;
    data.des = self.dom_des.val();
    return data;
}

return Com;