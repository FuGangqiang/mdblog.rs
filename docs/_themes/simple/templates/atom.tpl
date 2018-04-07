<?xml version="1.0" encoding="utf-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <id>{{ config.site_url }}</id>
  <title>{{ config.site_name }}</title>
  <updated>{{ now | date(format="%Y-%m-%dT%H:%M:%S%:z") }}</updated>
  <subtitle>{{ config.site_motto }}</subtitle>
  <icon>/static/favicon.png</icon>
  <logo>/static/logo.png</logo>
  <link rel="alternate" type="text/html" href="{{ config.site_url }}" />
  <link rel="self" type="application/atom+xml" href="{{ config.site_url }}/atom.xml" />
  <generator uri="https://github.com/FuGangqiang/mdblog.rs">mdblog.rs</generator>
  {% for post in posts -%}
  <entry>
    <id>{{ config.site_url }}{{ post.url  | urlencode }}</id>
    <title>{{ post.title }}</title>
    <updated>{{ post.headers.created | date(format="%Y-%m-%dT%H:%M:%S%:z") }}</updated>
    <published>{{ post.headers.created | date(format="%Y-%m-%dT%H:%M:%S%:z") }}</published>
    <link href="{{ config.site_url }}{{ post.url  | urlencode }}"/>
    <summary>{{ post.headers.description }}</summary>
    <content type="html" xml:lang="en" xml:base="{{ config.site_url }}">
        <![CDATA[
        {{ post.content }}
        ]]>
    </content>
  </entry>
  {%- endfor %}
</feed>
