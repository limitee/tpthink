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
    self.dom_bk_content = self.parent.find('#bk_content');
    self.dom_in_date = self.parent.find('#in_date');
    self.dom_out_date = self.parent.find('#out_date');
    self.dom_room_type_id = self.parent.find('#room_type_id');
    self.dom_room_count = self.parent.find('#room_count');
    self.dom_people_count = self.parent.find('#people_count');
    self.dom_name = self.parent.find('#name');
    self.dom_phone = self.parent.find('#phone');
    self.dom_extra = self.parent.find('#extra');

    self.dom_add_order = self.parent.find('#add_order');
    self.dom_add_order.on("click", function(e) {
        var data = self.get_data();
        CurSite.postUnDigest({cmd:"GO01"}, {data:data}, function(err, data){
            if(data) {
                var html = '<div class="container-fluid">' 
                         + '已提交成功，我们的服务人员会主动联系您！' + '</div>';
                self.dom_bk_content.html(html);
            }
        });
    });

    CurSite.postUnDigest({cmd:"GORT01"}, {}, function(err, data){
        if(data) {
            var html = '';
            var sets = data.data;
            for(var i = 0; i < sets.length; i++) {
                var set = sets[i];
                html += '<option value=' + set.id + '>' + set.name + '(' + set.price + '元)' + '</option>';
            }
            self.dom_room_type_id.html(html);
        }
    });
}

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.in_date = self.dom_in_date.val();
    data.out_date = self.dom_out_date.val();
    data.room_type_id = parseInt(self.dom_room_type_id.val());
    data.room_count = parseInt(self.dom_room_count.val());
    data.people_count = parseInt(self.dom_people_count.val());
    data.name = self.dom_name.val();
    data.phone = self.dom_phone.val();
    data.extra = self.dom_extra.val();
    return data;
}

return Com;
