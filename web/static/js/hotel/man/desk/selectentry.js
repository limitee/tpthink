var Com = function(config) {
    var self = this;
    self.config = config;
    self.parent = self.config.parent;
    //子页面
    self.cr = {
        'content': {
            pins: self,
            parent: self.parent.children('#content')
        }
    };
    self.page_list = [];    //页面列表
    self.cur_index = 0;
    self.init();
}

Com.prototype.init = function() {
    var self = this;
    self.dom_bread_cup = self.parent.find("#bread_cup");

    var page = {
        "url": "hotel_man_desk_selectdesk",
        "name": "选桌",
        "add": {
        }
    }
    self.page_list.push(page);

    self.refresh();
}

Com.prototype.refresh = function() {
    var self = this;
    var html = '';
    for(var i = 0; i < self.page_list.length; i++) {
        var page = self.page_list[i];
        html += '<li><a href="#" index="' + i + '">' + page.name + '>></a></li>';
    }
    self.dom_bread_cup.html(html);

    self.cur_index = self.page_list.length - 1;
    var last = self.page_list[self.cur_index];
    self.go_to(last);
    self.init_bread_event();
}

Com.prototype.init_bread_event = function() {
    var self = this;
    self.dom_bread_cup.find("li").on("click", function(e) {
        var index = parseInt($(this).children("a").attr("index"));
        if(index == self.cur_index) {
            return;
        }
        else
        {
            var splice_len = self.page_list.length - index - 1;
            self.page_list.splice(index + 1, splice_len);
            self.refresh();
        }
    })
}

Com.prototype.append = function(page) {
    var self = this;

    self.page_list.push(page);
    self.refresh();
}

Com.prototype.go_to = function(page) {
    var self = this;
    self.cr.content.add = page.add;
    CurSite.to_page(self.cr.content, page.url);
}

return Com;

