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
    var dom_user_list = self.parent.find('#user_list');
    CurSite.postDigest({cmd:"F01"}, {}, function(err, back_body)
    {
        dom_user_list.html(self.get_table(back_body.data));
    });
};

Com.prototype.get_table = function(data) {
    var self = this;
    var html = '<table class="table table-striped table-bordered table-hover">';
    html += '<thead><tr><td>名称</td><td>类型</td><td>大小</td><td>创建时间</td></tr></thead>';
    html += '<tbody>';
    for(var i = 0; i < data.length; i++) {
        html += '<tr><td>' + data[i].name + '</td><td>' + data[i].type + '</td><td>' + data[i].size + '</td><td>' + CurSite.getDateStr(data[i].create_time*1000) + '</td></tr>';
    }
    html += '</tbody>';
    html += '</table>';
    return html;
}

return Com;
