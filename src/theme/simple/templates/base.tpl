<!doctype html>
<html>
<head>
  <meta charset="utf-8">
  <link rel="icon" href="/static/img/favicon.png">
  <link rel="stylesheet" href="/static/css/main.css">
  <title>{{ title }}</title>
</head>
<body>
<header class="clearfix">
  <section id="imglogo">
    <a href="/index.html" title="Fu"><img src="/static/img/logo.png"></a>
  </section>

  <section id="textlogo">
    <h1 id="site-name"><a href="/index.html" title="Fu">Fu</a></h1>
    <h2 id="site-motto">Simple is Beautiful!</h2>
  </section>

  <nav>
    <ul>
      <li><a href="/index.html">Archives</a></li>
      <li><a href="/blog/modified.html">Last Modified</a></li>
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
        <li><a href="{{ tag.url }}">{{ tag.name }}<sup>{{ tag.num }}</sup></a></li>
      {% endfor %}
      </ul>
    </section>

    <section class="links clearfix">
      <h1>Links</h1>
      <ul>
        <li><a href="/index.html" target="_blank">Blog</a></li>
      </ul>
    </section>
  </aside>
</div>

<footer>
  <p>
    Keep It Simple, Stupid!
  </p>
</footer>
{% block math %}{% endblock math %}
</body>
</html>
