<rss version="2.0">
    <channel>
        <title>{{ config.site_name }}</title>
        <link>{{ config.site_url }}</link>
        <description>{{ config.site_motto }}</description>
        <generator>mdblog.rs</generator>
        {% for post in posts -%}
        <item>
            <title>{{ post.title }}</title>
            <link>{{ config.site_url }}{{ post.url  | urlencode }}</link>
            <description>{{ post.headers.description }}</description>
        </item>
        {%- endfor %}
    </channel>
</rss>
