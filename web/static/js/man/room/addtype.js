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
    self.dom_price = self.parent.find("#price");
    self.dom_sub.on("click", function(e) {
        self.dom_sub.button("loading");
        var data = self.get_data(); 
        self.save(data);
    });
};

Com.prototype.save = function(data) {
    var self = this;
    var body = {
        data:data
    }
    CurSite.postDigest({cmd:"ART02"}, body, function(err, back_body)
    {
        self.dom_sub.button("reset");
        self.parent.html("操作成功")
    });
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.name = self.dom_name.val();
    data.price = parseInt(self.dom_price.val());
    return data;
}

return Com;
