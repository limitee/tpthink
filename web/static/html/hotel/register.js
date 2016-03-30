var Com = function(config) {
    var self = this;
    self.config = config;
    //子页面
    self.cr = {
    };
    self.init();
};

Com.prototype.init = function() {
    var self = this;
    self.dom_username = $("#username");
    self.dom_password = $("#password");
    self.dom_owner = $("#owner");
    self.dom_owner_phone = $("#owner_phone");
    self.dom_hotel_name = $("#hotel_name");
    self.dom_hotel_addr = $("#hotel_addr");

    self.dom_reg_bt = $("#reg_bt");

    self.dom_reg_bt.on("click", function(e){
        var data = self.get_form_data();
        if(!self.check(data))
        {
            return;
        }
        CurSite.postUnDigest({cmd:"H01"}, data, function(err, data){
            if(err) {
                alert(err);
            }
            else
            {
                alert("注册成功");
            }
        });
    });
};

Com.prototype.check = function(data) {
    var self = this;
    if(data.username.length === 0)
    {
        alert("用户名格式不正确");
        return false;
    }
    if(data.password.length === 0)
    {
        alert("密码格式不正确");
        return false;
    }
    if(data.owner.length === 0)
    {
        alert("联系人格式不正确");
        return false;
    }
    if(data.owner_phone.length === 0)
    {
        alert("联系人电话格式不正确");
        return false;
    }
    if(data.hotel_name.length === 0)
    {
        alert("餐馆名称格式不正确");
        return false;
    }
    if(data.hotel_addr.length === 0)
    {
        alert("餐馆地址格式不正确");
        return false;
    }
    return true;
}

Com.prototype.get_form_data = function() {
    var self = this;
    var data = {};
    data.username = self.dom_username.val();
    data.password = self.dom_password.val();
    data.owner = self.dom_owner.val();
    data.owner_phone = self.dom_owner_phone.val();
    data.hotel_name = self.dom_hotel_name.val();
    data.hotel_addr = self.dom_hotel_addr.val();
    return data;
};

return Com;
