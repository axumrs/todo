# 使用axum构建todo服务

本专题将带你从零开始实现一个简单的、RESTFUL 风格的 Todo 服务。包括：JSON 响应及请求、PostgreSQL 的使用、自定义错误的处理、RESTFul 的定义、配置文件、日志的记录等。

> 在此之前，我们强烈建议你先阅读《[漫游 axum](https://axum.rs/subject/roaming-axum)》。

## 模型

我们的 Todo 服务分为两个模型：

- `TodoList`：待办事项

- `TodoItem`：待办事项的项目

## API

我们的 Todo 服务提供以下 API：

| 请求方式 | 路由                            | 说明                               |
| -------- | ------------------------------- | ---------------------------------- |
| `GET`    | `/todo`                         | 所有 TodoList                      |
| `POST`   | `/todo`                         | 为指定的 TodoList 添加 Item        |
| `GET`    | `/todo/:list_id`                | 获取 TodoList 详情                 |
| `DELETE` | `/todo/:list_id`                | 删除指定的 TodoList 及其 Item      |
| `PUT`    | `/todo/:list_id`                | 修改 TodoList                      |
| `GET`    | `/todo/:list_id/items`          | 获取 TodoList 关联的 Item          |
| `GET`    | `/todo/:list_id/items/:item_id` | 获取 TodoList 关联的某个 Item 详情 |
| `PUT`    | `/todo/:list_id/items/:item_id` | 修改 TodoList 关联的某个 Item 详情 |
| `DELETE` | `/todo/:list_id/items/:item_id` | 删除 TodoList 关联的某个 Item 详情 |

## 准备工作

开始之前，请先创建 PostgreSQL，并导入以下 SQL 语句：

```sql
DROP TABLE IF EXISTS todo_item;
DROP TABLE IF EXISTS todo_list;


CREATE TABLE todo_list(
    id SERIAL PRIMARY KEY,
    title VARCHAR(150) NOT NULL
);

CREATE TABLE todo_item(
    id SERIAL PRIMARY KEY,
    title VARCHAR(150) NOT NULL,
    checked BOOLEAN NOT NULL DEFAULT FALSE,
    list_id INTEGER NOT NULL,
    FOREIGN KEY(list_id) REFERENCES todo_list(id)
);

INSERT INTO todo_list (title) VALUES ('清单1'), ('清单2');

INSERT INTO todo_item (title, list_id) VALUES
    ('清单项目 1', 1),
    ('清单项目 2', 1),
    ('清单项目 A', 2);
```

## 代码

本专题代码可以在[axumrs/todo](https://github.com/axumrs/todo)找到。并且，每一章节的代码都以独立分支形式提供。

