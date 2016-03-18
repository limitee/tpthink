var Com = function(config) {
    var self = this;
    self.config = config;
    //子页面
    self.cr = {
        'main': {
            pins: self,
            parent: $("#content")
        }
    };
    self.init();
};

Com.prototype.init = function() {
    var self = this;
    self.temp_dom = $("#temp");
    self.detail_dom = $("#detail");
    self.wind_dom = $("#wind");

    $("a.list-item").click(function(event) {
        $(this).addClass("active").parent().siblings().find(".active").removeClass("active");
        var url = $(this).attr("tUrl");
        CurSite.to_page(self.cr.main, url);
    });

    CurSite.postUnDigest({cmd:"W01"}, {}, function(err, data){
        if(data) {
            self.temp_dom.html(data.temp);
            self.detail_dom.html(data.detail);
            self.wind_dom.html(data.wind);
        }
    });

    CurSite.to_page(self.cr.main, "nor_index");
};

return Com;
