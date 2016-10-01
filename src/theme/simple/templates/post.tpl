{% extends "base.tpl" %}


{% block main %}
    <h1>{{ title }}</h1>
    <article>
      {{ content }}

      <ul id="article_footer">
        {% if post_tags %}
           <li>tags: {% for tag in post_tags %}<a href="{{ tag.url }}">{{ tag.name }}<sup>{{ tag.num }}</sup></a>{% endfor %}</li>
        {% endif %}
        {% if datetime %}
           <li>date: {{ datetime }}</li>
        {% endif %}
      </ul>
    </article>
{% endblock main %}


{% block css %}
<link rel="stylesheet" href="/static/css/highlight.css">
{% endblock css %}


{% block js %}
<script src="/static/js/highlight.js"></script>
<script>hljs.initHighlightingOnLoad();</script>
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
