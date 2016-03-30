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
};

Com.prototype.save = function(data) {
    var self = this;
    var body = data;
    CurSite.postDigest({cmd:"HF01"}, body, function(err, back_body)
    {
        self.dom_sub.button("reset");
        self.parent.html("操作成功")
    });
}

Com.prototype.check = function(data) {
    if(data.name.length === 0)
    {
        alert("必须填分组的名称");
        return false;
    }
    return true;
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.name = self.dom_name.val();
    return data;
}

return Com;
