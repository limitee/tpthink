var CurSite = new function(){};
CurSite.host = window.document.location.protocol + "//" + window.document.location.hostname;
CurSite.site = CurSite.host;
var port = window.document.location.port;
if(port && port.length > 0)
{
    CurSite.site +=  ":" + port;
}
CurSite.getContextPath = function() {
    return "";
};
CurSite.loadFlag = {};  //组件加载标识
CurSite.conextPath = CurSite.getContextPath();
CurSite.getAbsolutePath = function(url) {
    return CurSite.site + CurSite.conextPath + "/" + url;
};
CurSite.interPath = CurSite.getAbsolutePath("filter/interface.htm");

CurSite.COMP = {};  //组件定义
CurSite.IMPL = {};  //组件实现

CurSite.fileSite = "http://file.versou.com:8081";

CurSite.createUUID = function() {
    var s = [];
    var hexDigits = "0123456789abcdef";
    for (var i = 0; i < 36; i++) {
        s[i] = hexDigits.substr(Math.floor(Math.random() * 0x10), 1);
    }
    s[14] = "4";  // bits 12-15 of the time_hi_and_version field to 0010
    s[19] = hexDigits.substr((s[19] & 0x3) | 0x8, 1);  // bits 6-7 of the clock_seq_hi_and_reserved to 01
    s[8] = s[13] = s[18] = s[23] = "";
    var uuid = s.join("");
    return uuid;
};
/**
 * 格式化int，不够的位置�?0
 * @param value
 * @param len
 */
CurSite.formatInt= function(value, len)
{
    var str = value + "";
    var prefix = "";
    for(var i = 0; i < len - str.length; i++)
    {
        prefix += "0";
    }
    return prefix + str;
};
CurSite.getDateStr = function(m_sec)
{
    if(m_sec) {
        var date = new Date(m_sec);
    }
    else
    {
        var date = new Date();
    }
    var str = "";
    str += date.getFullYear();
    str += "-";
    str += CurSite.formatInt(date.getMonth() + 1, 2);
    str += "-";
    str += CurSite.formatInt(date.getDate(), 2);
    str += " ";
    str += CurSite.formatInt(date.getHours(), 2);
    str += ":";
    str += CurSite.formatInt(date.getMinutes(), 2);
    str += ":";
    str += CurSite.formatInt(date.getSeconds(), 2);
    str += ".";
    str += CurSite.formatInt(date.getMilliseconds(), 3);
    return str;
};

CurSite.arrayBufferToBase64 = function(buffer) {
    var binary = '';
    var bytes = new Uint8Array( buffer );
    var len = bytes.byteLength;
    for (var i = 0; i < len; i++) {
        binary += String.fromCharCode(bytes[i]);
    }
    return window.btoa(binary);
}

CurSite.getDefualtKey = function()
{
    return "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
};

CurSite.getDefaultIv = function()
{
    return "AAAAAAAAAAA=";
};

String.prototype.trim = function() {
    return this.replace(/(^\s*)|(\s*$)/g, '');
}

/**
 * get cookies
 */
CurSite.getCookie = function()
{
    var cookieStr = document.cookie;
    var cookieStrArray = cookieStr.split(";");
    var siteCookies = {};
    for(var index in cookieStrArray)
    {
        if(cookieStrArray[index].length > 0)
        {
            var cookieArray = cookieStrArray[index].split("=");
            var key = cookieArray[0].trim();
            var value = cookieArray[1].trim();
            siteCookies[key] = value;
        }
    }
    return siteCookies;
};

CurSite.setCookie = function(name, value, expSeconds)
{
    var exp = new Date();
    if(expSeconds < 0)  //负数，表示不过期
    {
        expSeconds = 1000000;
    }
    exp.setTime(exp.getTime() + expSeconds*1000);
    document.cookie = name + "="+ value + ";expires=" + exp.toGMTString() + ";path=/";
};


/**
 * 接口数据进行加密
 * @param headNode
 * @param key
 * @param bodyStr
 * @returns {{head: *, body: string}}
 */
CurSite.encrypt = function(headNode, key, bodyStr)
{
    if(headNode.digestType == "md5-empty")
    {
        key = CurSite.getDefualtKey();
    }
    var key_content = key + bodyStr + headNode.timeStamp;
    var hash = CryptoJS.MD5(key_content);
    headNode.digest = hash.toString(CryptoJS.enc.Hex);
    return {head:JSON.stringify(headNode), body:bodyStr};
};

/**
 * 接口数据进行解密
 * @param headNode
 * @param key
 * @param encodedBodyStr
 * @returns {*}
 */
CurSite.decrypt = function(headNode, key, encodedBodyStr)
{
    if(headNode.digestType == "md5-empty")
    {
        key = CurSite.getDefualtKey();
    }
    var key_content = key + encodedBodyStr + headNode.timeStamp;
    var hash = CryptoJS.MD5(key_content).toString(CryptoJS.enc.Hex);
    if(hash == headNode.digest) {
        return JSON.parse(encodedBodyStr);
    }
    return null;
};

/**
 * 获取指定url的html内容
 * @param url
 * @param params 要发送到服务器的参数
 * @param cb
 */
CurSite.get_html = function(url, params, cb)
{
    var self = this;
    $.ajax({
        url:url,
        type:'GET',
        success:function(data) {
            cb(null, data);
        },
        error : function() {
            cb("网络异常", "网络异常");
        }
    });
};

CurSite.postUnDigest = function(head, body, cb)
{
    var self = this;
    head.digestType = 'md5-empty';
    head.userId = '';
    head.userType = 'GUEST';
    head.timeStamp = self.getDateStr();
    body.uuid = CurSite.createUUID();   //消息的id
    var bodyStr = JSON.stringify(body);
    var msgNode = CurSite.encrypt(head, null, bodyStr);
    $.ajax({
        url:"./api/data",
        data:msgNode,
        type:'post',
        cache:false,
        dataType:'json',
        success:function(data) {
            var backBodyStr = data.body;
            var back_body = CurSite.decrypt(data.head, null, backBodyStr);
            if(back_body.err)
            {
                cb(back_body.err, back_body);
            }
            else
            {
                cb(null, back_body);
            }
        },
        error : function() {
            cb("网络错误", null); //unknown error
        }
    });
};

CurSite.postDigest = function(head, body, cb)
{
    var self = this;
    var cookies = self.getCookie();
    if(head.key)
    {
        var key = head.key;
        delete head.key;
    }
    else
    {
        var key = cookies["st"];
    }
    head.digestType = 'md5';
    head.userId = cookies["userId"];
    head.userType = cookies["userType"];
    head.timeStamp = self.getDateStr();
    body.uuid = CurSite.createUUID();   //消息的唯一id
    var bodyStr = JSON.stringify(body);
    var msgNode = CurSite.encrypt(head, key, bodyStr);
    $.ajax({
        url:"./api/data",
        data:msgNode,
        type:'post',
        cache:false,
        dataType:'json',
        success:function(data) {
            var backBodyStr = data.body;
            var back_body = CurSite.decrypt(data.head, key, backBodyStr);
            if(back_body.err)
            {
                cb(back_body.err, back_body);
            }
            else
            {
                cb(null, back_body);
            }
        },
        error : function() {
            cb("网络错误", null); //unknown error
        }
    });
};

CurSite.get_js = function(path, cb)
{
    var api_path = "./api/js/" + path;
    $.ajax({
        url:api_path,
        type:'get',
        success:function(data) {
            cb(null, data);
        },
        error : function() {
            cb({code:-1, description:"网络错误"}, null); //unknown error
        }
    });
};

CurSite.to_page = function(config, page, cb) {
    var js_path = page.split("_").join("/") + ".js";
    var html_path = page + ".html";
    CurSite.get_html(html_path, {}, function(err, data){
        config.parent.html(data);
        CurSite.get_js(js_path, function(err, data) {
            var f = new Function(data);
            config.DEFINE = f();
            config.ins = new config.DEFINE({parent:config.parent, pins:config.pins, add:config.add});
            if(cb) {
                cb(err, null);
            }
        });
    });
};

//mix the src's properties to target
CurSite.mix = function(target, src) {
    for(var key in src) {
        target[key] = src[key];
    }
}
