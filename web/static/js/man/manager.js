var Com = function(config) {
    var self = this;
    self.config = config;
    //子页面
    self.cr = {
        'main': {
            pins: self,
            parent: $('#content')
        }
    };
    self.init();
}

Com.prototype.init = function() {
    var self = this;

    self.dom_room_type_list = $('#doc_list');
    self.dom_room_type_list.on('click', function(e) {
        CurSite.to_page(self.cr.main, "man_doc_list");
    });

    //CurSite.to_page(self.cr.main, "man_hotel_list");
}

return Com;
