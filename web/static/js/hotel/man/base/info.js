var Com = function(config) {
	var self = this;
	self.config = config;
    self.parent = self.config.parent;
	//子页面
	self.cr = {
	};
	self.init();
};

Com.prototype.init = function() {
	var self = this;
    self.dom_username = self.parent.find("#username");
    self.dom_password = self.parent.find("#password");
    self.dom_name = self.parent.find("#name");
    self.dom_addr = self.parent.find("#addr");

    var body = {
    }
    CurSite.postDigest({cmd:"H02"}, body, function(err, back_body)
    {
        if(back_body)
        {
            var customer = back_body.customer;
            self.dom_username.html(customer.username);
            self.dom_password.html(customer.password);
            
            var hotel = back_body.hotel;
            self.dom_name.html(hotel.name);
            self.dom_addr.html(hotel.addr);
        }
    });
};

return Com;
