var Com = function() {
    var self = this;
    self.data = {
    };
}

Com.prototype.get_event_list = function(cb) {
    var self = this;
    el = [];
    cb(null, el);
}

