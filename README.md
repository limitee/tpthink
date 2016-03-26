<<<<<<< HEAD
# order_sys
订餐系统

ssh root@123.57.64.210
3edcvBHU8

```
scp ./target/release/web root@123.57.64.210:/data/lottchina/workspace/lot/web/target/release/.
```

```
nohup ./web target=run name=order_sys > nohup.out 2>&1 &
```

```
/usr/local/pgsql/bin/initdb -D /usr/local/pgsql/data
```

```
/usr/local/pgsql/bin/createdb order_sys
```

#添加用户
```
CREATE ROLE postgres superuser;
```

#修改密码
```
ALTER ROLE postgres WITH PASSWORD 'bb3RrH8nrwUtN4eq';
```

#赋予登录权限
```
ALTER ROLE postgres WITH login;
```

导出
```
/usr/local/pgsql/bin/pg_dump order_sys > ./order_sys.sql
```

导入
```
/usr/local/pgsql/bin/psql order_sys < ./order_sys.sql
```

停止数据库
```
/usr/local/pgsql/bin/pg_ctl -D /usr/local/pgsql/data stop
```

创建索引
```
CREATE UNIQUE INDEX order_food_unique ON order_food (customer_id, forder_id, food_id);
```

# lot
lot system

LD_PRELOAD=/lib/libc.so.6.back ls
=======
# tpthink
tipthink site
>>>>>>> 635601f382edcaf1c732fded66504caf052cf2e9
