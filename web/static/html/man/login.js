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
    self.dom_reg_bt = $("#reg_bt");

    self.dom_reg_bt.on("click", function(e){
        var data = self.get_data();
        CurSite.setCookie("userId", data.username, -1);
        CurSite.setCookie("userType", "admin", -1);
        var key = CryptoJS.MD5(data.password).toString(CryptoJS.enc.Hex);
        CurSite.postDigest({cmd:"A01", key:key}, {}, function(err, data){
            if(err) {
                alert(err);
            }
            else
            {
                CurSite.setCookie("userId", data.userId, -1);
                CurSite.setCookie("st", data.st, -1);

                window.location = "./man_manager.html";
            }
        });
    });
};

Com.prototype.get_data = function() {
    var self = this;
    var data = {};
    data.username = self.dom_username.val();
    data.password = self.dom_password.val();
    return data;
}

return Com;
