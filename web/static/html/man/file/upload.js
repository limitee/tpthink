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
    self.dom_choose = self.parent.find("#choose");
    self.dom_choose_file = self.parent.find("#choose_file");
    self.dom_choose.on("click", function(e) {
        self.dom_choose_file.click();
    });
    self.dom_file_pg = self.parent.find("#file_pg");
    self.dom_choose_file.on("change", function(e) {
        self.dom_choose.button("loading");
        
        var file = self.dom_choose_file.get(0).files[0];
        console.log(file);
        var start = function(file, cb) {
            fileType = file.type;
            var body = {name:file.name, size:file.size, type:fileType};
            CurSite.postDigest({cmd:"F02"}, body, function(err, back_body)
            {
                cb(err, back_body);
            })
        }
        start(file, function(err, data){
            if(data) {
                var file_id = data.id;
                var file_reader = new FileReader();
                var cur = 0;    //当前读取进度
                var blockSize = 1024*20;   //读取的文件块大小,10KB
                var percent = 0;    //当前文件读取百分比进度
                var fileSize = file.size;   //文件大小
                var index = 1;
                var read_more = function() {
                    var sendSize = blockSize;
                    if(cur + blockSize >= fileSize)
                    {
                        sendSize = fileSize - cur;
                    }
                    console.log("文件大小:" + fileSize + ",cur:" + cur + ",send:" + sendSize);
                    var newFile = file.slice(cur, cur + sendSize);
                    file_reader.readAsBinaryString(newFile);
                };
                var update_pg = function() {
                    percent = parseInt(cur*100/fileSize);
                    if(percent > 100)
                    {
                        percent = 100;
                    }
                    self.dom_file_pg.text(percent + "%");
                    self.dom_file_pg.css("width", percent + "%");
                };
                var finish = function() {
                    self.dom_file_pg.text(file.name + "已经成功上传"); 
                    self.dom_choose.button("reset");
                };
                file_reader.onload = function(event) {
                    var str = window.btoa(event.target.result);
                    var body = {index:index, start:cur, size:event.total, file_id:file_id, content:str};
                    CurSite.postDigest({cmd:"F03"}, body, function(err, back_body)
                    {
                        cur += blockSize;
                        index++;
                        update_pg();
                        if(cur < fileSize) {
                            read_more();
                        } else {
                            finish();
                        }
                    })
                };
                read_more();
            } else {
                alert(err);
            }
        })
    })
};

return Com;
