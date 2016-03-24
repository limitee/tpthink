var Com = function(config) {
    var self = this;
    self.config = config;
    self.parent = self.config.parent;
    //子页面
    self.cr = {
    };
    self.init();
}

Com.prototype.init = function() {
    var self = this;
    self.dom_content = self.parent.find("#content");
    self.dom_title = self.parent.find("#title");
    var args = CurSite.getPathArgs();
    self.doc_id = parseInt(args.id);
    var body = {
        id: self.doc_id
    }
    CurSite.postDigest({cmd:"AD03"}, body, function(err, back_body)
    {
        if(err) {
            return;
        }
        var set = back_body.data[0];
        self.dom_title.html(set.title);
        self.dom_content.html(mk.to_html(set.content));
    });
}

return Com;

