{% extends "base.tpl" %}

{% block title %}
  <title>{{ post.title }}</title>
{% endblock title %}

{%- block css %}{% endblock css -%}

{% block main %}
    <h1>{{ post.title }}</h1>
    <article>
      {{ post.content }}
      <ul id="article_footer">
      {%- if post_tags %}
        <li>tags: {% for tag in post_tags %}<a href="{{ config.site_url }}{{ tag.url }}">{{ tag.name }}<sup>{{ tag.num }}</sup></a>{% endfor %}</li>
      {% endif -%}
        <li>date: {{ post.headers.created | date(format="%Y-%m-%d %H:%M:%S") }}</li>
      </ul>
    </article>
{%- endblock main %}

{% block js %}
<script src="{{ config.site_url }}/static/main.js"></script>
<script src="//cdn.mathjax.org/mathjax/latest/MathJax.js?config=TeX-AMS-MML_HTMLorMML"></script>
<script type="text/x-mathjax-config">
    MathJax.Hub.Config({
       tex2jax: {
          inlineMath: [ ['$','$'], ["\\(","\\)"] ],
          processEscapes: true,
          skipTags: ['script', 'noscript', 'style', 'textarea', 'pre', 'code']
       },
       TeX: {equationNumbers: {autoNumber: ["AMS"], useLabelIds: true}},
       "HTML-CSS": {linebreaks: {automatic: true}},
       SVG: {linebreaks: {automatic: true}}
    });
    MathJax.Hub.Queue(function() {
       var all = MathJax.Hub.getAllJax(), i;
       for(i=0; i < all.length; i += 1) {
          all[i].SourceElement().parentNode.className += ' has-jax';
       }
    });
</script>
{% endblock js %}
