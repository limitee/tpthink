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

    self.dom_room_type_list = $('#base_info');
    self.dom_room_type_list.on('click', function(e) {
        CurSite.to_page(self.cr.main, "hotel_man_base_info");
    });

    self.dom_entry = $('#entry');
    self.dom_entry.on('click', function(e) {
        CurSite.to_page(self.cr.main, "hotel_man_base_entry");
    });

    self.dom_my_files = $('#my_files');
    self.dom_my_files.on('click', function(e) {
        CurSite.to_page(self.cr.main, "man_file_list");
    });

    self.dom_upload_files = $('#upload_files');
    self.dom_upload_files.on('click', function(e) {
        CurSite.to_page(self.cr.main, "man_file_upload");
    });

    self.dom_desk_group = $('#desk_group');
    self.dom_desk_group.on('click', function(e) {
        CurSite.to_page(self.cr.main, "hotel_man_desk_addgroup");
    });

    self.dom_list_group = $('#list_group');
    self.dom_list_group.on('click', function(e) {
        CurSite.to_page(self.cr.main, "hotel_man_desk_listgroup");
    });

    self.dom_add_desk_ = $('#add_desk');
    self.dom_add_desk_.on('click', function(e) {
        CurSite.to_page(self.cr.main, "hotel_man_desk_adddesk");
    });

    self.dom_list_desk_ = $('#list_desk');
    self.dom_list_desk_.on('click', function(e) {
        CurSite.to_page(self.cr.main, "hotel_man_desk_listdesk");
    });

    self.dom_food_group = $('#food_group');
    self.dom_food_group.on('click', function(e) {
        CurSite.to_page(self.cr.main, "hotel_man_food_addgroup");
    });

    self.dom_food_list_group = $('#food_list_group');
    self.dom_food_list_group.on('click', function(e) {
        CurSite.to_page(self.cr.main, "hotel_man_food_listgroup");
    });

    self.dom_add_food = $('#add_food');
    self.dom_add_food.on('click', function(e) {
        CurSite.to_page(self.cr.main, "hotel_man_food_addfood");
    });

    self.dom_list_food = $('#list_food');
    self.dom_list_food.on('click', function(e) {
        CurSite.to_page(self.cr.main, "hotel_man_food_listfood");
    });

    CurSite.to_page(self.cr.main, "hotel_man_base_info");
}

return Com;
