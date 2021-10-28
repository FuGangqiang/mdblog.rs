{% extends "base.tpl" %}

{% block title %}
  <title>{{ post.title }}</title>
{% endblock title %}

{%- block css %}{% endblock css -%}

{% block main %}
    <h1>{{ post.title }}</h1>
    <article>
      {{ post.content }}
    </article>
    <div id="article-footer">
      {%- if post.headers.tags %}
        <div>
          <svg class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"><path d="M323.008 786.752c-52.928 0-96-43.072-96-96s43.072-96 96-96 96 43.072 96 96S375.936 786.752 323.008 786.752zM323.008 658.752c-17.632 0-32 14.336-32 32s14.368 32 32 32 32-14.336 32-32S340.64 658.752 323.008 658.752z" p-id="16156" fill="#bfbfbf"></path><path d="M416.096 927.072 284.224 927.072c-159.936 0-186.912-59.232-186.912-192l0-140.8c0-74.272 14.304-96.256 70.72-150.976l327.04-319.904c36.576-35.488 105.888-35.392 142.304-0.096l263.072 256.032c18.336 17.792 28.864 43.552 28.864 70.656 0 27.296-10.656 53.28-29.248 71.264l-290.016 294.592C544.544 880.416 497.216 927.072 416.096 927.072zM566.24 159.488c-10.496 0-20.16 3.52-26.528 9.696l-327.04 319.936c-49.952 48.48-51.36 54.528-51.36 105.152l0 140.8c0 110.272 8.352 128 122.912 128l131.872 0c52.672 0 83.744-28.48 148.992-92.8l26.656-26.144 263.232-268.256c6.784-6.592 10.336-15.808 10.336-25.888 0-9.888-3.424-18.88-9.472-24.736l-263.072-256.032C586.432 163.04 576.736 159.488 566.24 159.488z"></path></svg>
          {% for name in post.headers.tags %}<a href="{{ config.site_url }}/tags/{{ name | urlencode }}.html">{{ name }}<sup>{{ tag_map[name].num }}</sup></a>{% endfor %}
        </div>
      {% endif -%}
        <div>
          <svg class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg"><path d="M512 192c179.2 0 320 140.8 320 320s-140.8 320-320 320-320-140.8-320-320S332.8 192 512 192M512 128C300.8 128 128 300.8 128 512s172.8 384 384 384 384-172.8 384-384S723.2 128 512 128L512 128z" p-id="1937" fill="#8a8a8a"></path><path d="M640 672c-6.4 0-19.2 0-25.6-6.4l-128-128C486.4 531.2 480 518.4 480 512L480 288C480 268.8 492.8 256 512 256s32 12.8 32 32l0 211.2 121.6 121.6c12.8 12.8 12.8 32 0 44.8C659.2 672 646.4 672 640 672z" p-id="1938"></path></svg>
          {{ post.headers.created | date(format="%Y-%m-%d %H:%M:%S") }}
        </div>
      </div>
{%- endblock main %}

{% block js %}
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/katex@0.13.20/dist/katex.min.css" integrity="sha384-cRxb1HsKSl8bTfU9fBcGsjktUfQa6w+fwvkYnU8XjFH4Qg8To1+/9OXv5iRzrKX4" crossorigin="anonymous">
<script defer src="https://cdn.jsdelivr.net/npm/katex@0.13.20/dist/katex.min.js" integrity="sha384-ov99pRO2tAc0JuxTVzf63RHHeQTJ0CIawbDZFiFTzB07aqFZwEu2pz4uzqL+5OPG" crossorigin="anonymous"></script>
<script>
  "use strict";
  document.addEventListener("DOMContentLoaded", function () {
    var maths = document.getElementsByClassName("language-math");
    for (var i = 0; i < maths.length; i++) {
      var el = maths[i];
      katex.render(el.innerText, el, { displayMode: true });
    }
    var codes = document.getElementsByTagName("code");
    for (i = 0; i < codes.length; i++) {
      el = codes[i];
      if (el.classList.contains("language-math")) continue;
      if (el.classList.contains("language-inline-math")) {
        katex.render(el.innerText, el);
        continue;
      }
      var n = el.nextSibling;
      var p = el.previousSibling;
      if (n && p && /^\$/.test(n.data) && /\$$/.test(p.data)) {
        katex.render(el.innerText, el);
        n.splitText(1);
        n.remove();
        p.splitText(p.data.length - 1).remove();
      }
    }
  });
</script>
{% endblock js %}
