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
        var data = self.get_form_data();
        CurSite.postUnDigest({cmd:"U01"}, data, function(err, data){
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

Com.prototype.get_form_data = function(){
    var self = this;
    var data = {};
    data.username = self.dom_username.val();
    data.password = self.dom_password.val();
    return data;
};

return Com;
