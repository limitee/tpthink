var Com = function() {
    var self = this;
    self.data = {
        user_list:[
            {name:"liming", age:123}
        ]
    };
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    el = [];
    cb(null, el);
}

Com.prototype.page_loaded = function(cb) {
    var self = this;
    var head_id = self.com.get_id("head");
    new window.Com({id:head_id, path:"index.head", pins:self}, function(err, c){
        console.log(c.com.pcom)
    });

    var el = [{id:self.com.get_jid("sbt"), on:"click", do:function(e){
        var new_user = {name:"test", age:10};
        self.data.user_list.push(new_user);
        self.snip_top.refresh()
    }}];
    var tpl =   '{@each user_list as user}' +
                    'hello, ${user.name}, your age is ${user.age}.' +
                '{@/each}' +
                '<button id="${index}_sbt" class="btn btn-default">提交</button>';
    self.snip_top = new Snip(self, "top", tpl, el);
}
