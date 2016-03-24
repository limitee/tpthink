var Com = function(config) {
    var self = this;
    self.config = config;
    self.parent = self.config.parent;
    if(self.config.add && self.config.add.doc_id) {
        self.doc_id = self.config.add.doc_id;
    }
    self.init();
};

Com.prototype.init = function() {
    var self = this;
    self.dom_title = self.parent.find("#title");
    self.dom_text_input = self.parent.find("#text_input");
    self.dom_preview = self.parent.find("#preview");

    if(self.doc_id) {
        var body = {
            id: self.doc_id
        }
        CurSite.postDigest({cmd:"AD03"}, body, function(err, back_body)
        {
            if(err) {
                return;
            }
            var set = back_body.data[0];
            self.dom_title.val(set.title);
            self.dom_text_input.val(set.content);
            //console.log(back_body);
        });
    }
    self.dom_text_input.on("input", function(e){
        var text = $(this).val();
        self.dom_preview.html(mk.to_html(text));
        //self.parent.scrollTop(self.parent.prop('scrollHeight'));
    });
    self.dom_sub = self.parent.find("#sub");
    self.dom_sub.on("click", function(e) {
        self.dom_sub.button("loading");
        var data = self.get_data();
        if(self.check(data)) {
            self.save(data);
        } else {
            self.dom_sub.button("reset");
        }
    });
    self.dom_preview.html(mk.to_html(self.dom_text_input.val()));
};

Com.prototype.save = function(data) {
    var self = this;
    var body = {
        data:data
    }
    CurSite.postDigest({cmd:"AD02"}, body, function(err, back_body)
    {
        self.dom_sub.button("reset");
        self.parent.html("操作成功")
    });
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    if(self.doc_id) {
        data.id = self.doc_id;
    }
    data.title = self.dom_title.val();
    data.content = self.dom_text_input.val();
    return data;
}

Com.prototype.check = function(data) {
    var self = this;
    if(data.title.length == 0) {
        alert("标题不能为空");
        return false;
    }
    if(data.content.length == 0) {
        alert("内容不能为空");
        return false;
    }
    return true;
}

return Com;
