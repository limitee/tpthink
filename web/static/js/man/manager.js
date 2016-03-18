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

    self.dom_room_type_list = $('#room_type');
    self.dom_room_type_list.on('click', function(e) {
        CurSite.to_page(self.cr.main, "man_room_typelist");
    });

    self.dom_add_room_type = $('#add_room_type');
    self.dom_add_room_type.on('click', function(e) {
        CurSite.to_page(self.cr.main, "man_room_addtype");
    });

    self.dom_my_files = $('#hotel_list');
    self.dom_my_files.on('click', function(e) {
        CurSite.to_page(self.cr.main, "man_hotel_list");
    });

    CurSite.to_page(self.cr.main, "man_hotel_list");
}

return Com;
