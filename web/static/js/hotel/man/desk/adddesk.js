var Com = function(config) {
    var self = this;
    self.config = config;
    self.parent = self.config.parent;
    self.init();
};

Com.prototype.init = function() {
    var self = this;
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
    CurSite.postDigest({cmd:"HD02"}, body, function(err, back_body)
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
    CurSite.postDigest({cmd:"HD04"}, body, function(err, back_body)
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
    return true;
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.name = self.dom_name.val();
    data.group_id = parseInt(self.dom_group_id.val());
    return data;
}

return Com;
