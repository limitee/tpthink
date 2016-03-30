var Com = function(config) {
	var self = this;
	self.config = config;
    self.parent = self.config.parent;
    //子页面
    self.cr = {
        'main': {
            pins: self,
            parent: self.parent.children('#content')
        }
    };
	self.init();
};

Com.prototype.init = function() {
	var self = this;
    self.dom_nav_bar = self.parent.find("#nav_bar");
    CurSite.to_page(self.cr.main, "man_doc_list");

    self.dom_nav_bar.find("li").on("click", function(e) {
        $(this).parent().find("li.active").removeClass("active");
        var url = $(this).attr("url");
        $(this).addClass("active");

        self.cr.main.add = {};
        CurSite.to_page(self.cr.main, url);
    });
}

Com.prototype.edit_doc = function(doc_id) {
    var self = this;
    var add = {
        doc_id: doc_id
    }
    self.cr.main.add = add;
    CurSite.to_page(self.cr.main, "man_doc_add");
}

return Com;
