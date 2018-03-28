<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <meta name="generator" content="mdblog.rs">
  <link rel="icon" href="{{ url_prefix }}/static/favicon.png">
  <link rel="stylesheet" href="{{ url_prefix }}/static/main.css">
  {% block css %}{% endblock css %}
  <title>{{ title }}</title>
</head>
<body>
<header class="clearfix">
  <section id="imglogo">
    <a href="{{ url_prefix }}/index.html" title="{{ site_name }}">
    <img src="{{ url_prefix }}/static/logo.png"></a>
  </section>

  <section id="textlogo">
    <h1 id="site-name">
      <a href="{{ url_prefix }}/index.html" title="{{ site_name }}">{{ site_name }}</a>
    </h1>
    <h2 id="site-motto">{{ site_motto }}</h2>
  </section>

  <nav>
    <ul>
      <li><a href="{{ url_prefix }}/index.html">Blog</a></li>
    </ul>
  </nav>
</header>
<div id="container" class="clearfix">
  <main>
    {% block main %}{% endblock main %}
  </main>

  <aside>
    <section class="tags clearfix">
      <h1>Tags</h1>
      <ul>
      {% for tag in all_tags %}
        <li><a href="{{ url_prefix }}{{ tag.url }}">{{ tag.name }}<sup>{{ tag.num }}</sup></a></li>
      {% endfor %}
      </ul>
    </section>

    <section class="links clearfix">
      <h1>Links</h1>
      <ul>
        <li><a href="{{ url_prefix }}/index.html" target="_blank">Blog</a></li>
      </ul>
    </section>
  </aside>
</div>

<footer>
  <p>
    {{ footer_note }}
  </p>
</footer>
{% block js %}{% endblock js %}
</body>
</html>
