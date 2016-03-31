(function() {
var Com = function(cfg, cb) {
    var self = this;
    self.id = cfg.id
    self.cfg = cfg;
    self.pins = cfg.pins;
    self.cr = {};   //子页面
    if(self.pins) {
        self.pcom = self.pins.com;
        self.pcom.cr[self.id] = self;
    }
    self.parent = $("#" + cfg.id);
    self.index = window.com_index;  //全局控件索引
    window.com_index += 1;
    self.init(cb);
}

Com.prototype.init = function(cb) {
    var self = this;
    async.waterfall([
        function(cb){
            self.get_tpl(cb)
        },
        function(tpl, cb) {
            self.get_js(function(err, data){
                cb(err, tpl, data + "\nreturn Com;")
            })
        },
        function(tpl, js, cb) { //初始化控件定义
            self.tpl = tpl;
            var Define = new Function(js);
            self.Define = Define();
            self.ins = new self.Define();
            if(!self.ins.data) {
                self.ins.data = {};
            }
            self.ins.data.index = self.index;   //设置组件序号,单页面内组件的id应该是唯一的
            self.ins.com = self;
            if(self.ins.init) {
                self.ins.init(function(err, data){
                    cb(err);
                });
            } else {
                cb(null);
            }
        },
        function(cb) {  //渲染界面,并绑定事件
            self.refresh(function(err, data){
                cb(err)
            })
        }
    ], function(err, data){
        if(cb) {
            cb(err, self.ins);
        }
    })
}

Com.prototype.refresh = function(cb) {
    var self = this;
    async.waterfall([
        function(cb) {  //渲染界面,并绑定事件
            var data = self.ins.data;
            var html = juicer(self.tpl, data);
            var node = $(html);
            if(self.ins.get_event_list) {
                self.ins.get_event_list(function(err, event_list){
                    for(var key in event_list) {
                        var set = event_list[key];
                        node.find(set.id).on(set.on, set.do);
                    }
                    self.parent.html(node);
                    cb(null)
                })
            } else {
                self.parent.html(node);
                cb(null)
            }
        },
        function(cb) {      //页面加载完成事件
            if(self.ins.page_loaded) {
                self.ins.page_loaded(function(err, data){
                    cb(err)
                });
            } else {
                cb(null)
            }
        }
    ], function(err, data){
        if(cb) {
            cb(err, data);
        }
    })
}

Com.prototype.get_id = function(in_id) {
    var self = this;
    var id = self.index + "_" + in_id;
    return id;
}

Com.prototype.get_jid = function(in_id) {
    var self = this;
    var id = "#" + self.index + "_" + in_id;
    return id;
}

Com.prototype.get = function(in_id) {
    var self = this;
    var jid = self.get_jid(in_id);
    return self.parent.find(jid);
}

//get tpl from the server
Com.prototype.get_tpl = function(cb) {
    var self = this;
    var url = self.cfg.path + ".html";
    $.ajax({
        url:url,
        type:'GET',
        success:function(data) {
            cb(null, data);
        },
        error : function() {
            cb("网络异常", "网络异常", null);
        }
    });
}

Com.prototype.get_js = function(cb) {
    var self = this;
    var url = "./api/js/" + self.cfg.path.split("_").join("/") + ".js";
    $.ajax({
        url:url,
        type:'GET',
        success:function(data) {
            cb(null, data);
        },
        error : function() {
            cb("网络异常", "网络异常", null);
        }
    });
}

//需要使用部分刷新,而不是整个页面刷新时使用
var Snip = function(pins, id, tpl, event_list) {
    var self = this;
    tpl = '<div class="container-fluid no_space">' + tpl + '</div>';
    self.pins = pins;
    self.id = id;
    self.tpl = tpl;
    self.event_list = event_list;
    self.refresh();
}

Snip.prototype.refresh = function() {
    var self = this;
    var data = self.pins.data;
    data.index = self.pins.com.index;    //组件的序号,单页面内组件的id应该是唯一的
    var html = juicer(self.tpl, data);
    var node = $(html);
    for(var key in self.event_list) {
        var set = self.event_list[key];
        node.find(set.id).on(set.on, set.do);
    }
    self.pins.com.get(self.id).html(node);
}

window.Com = Com;
window.Snip = Snip;
window.com_index = 0;
})()