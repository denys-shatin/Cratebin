# SEO Оптимизация для Cratebin

## Что уже сделано

✅ **Meta теги** - title, description, keywords
✅ **Open Graph** - для соцсетей (Facebook, Twitter)
✅ **Schema.org** - структурированные данные для Google
✅ **Sitemap.xml** - карта сайта
✅ **Robots.txt** - инструкции для поисковиков
✅ **Canonical URL** - избежание дублей
✅ **Semantic HTML** - правильная структура
✅ **Footer с ключевыми словами**

## После деплоя

### 1. Google Search Console
```
1. Зайди на https://search.google.com/search-console
2. Добавь сайт cratebin.biz
3. Подтверди владение (через DNS или HTML файл)
4. Отправь sitemap: https://cratebin.biz/sitemap.xml
```

### 2. Google Analytics (опционально)
```
1. Создай аккаунт на https://analytics.google.com
2. Получи tracking ID (G-XXXXXXXXXX)
3. Добавь в frontend/src/app.html перед </head>:

<script async src="https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'G-XXXXXXXXXX');
</script>
```

### 3. Создай OG Image
Создай изображение 1200x630px с логотипом и текстом:
```
"Cratebin - Free Online Pastebin"
"Share code snippets with password protection"
```

Сохрани как `frontend/static/og-image.png`

### 4. Favicon
Создай иконки:
- `frontend/static/favicon.ico` (32x32)
- `frontend/static/apple-touch-icon.png` (180x180)

Можно использовать https://realfavicongenerator.net/

### 5. Отправь в поисковики

**Google:**
```
https://www.google.com/ping?sitemap=https://cratebin.biz/sitemap.xml
```

**Bing:**
```
https://www.bing.com/webmaster/ping.aspx?siteMap=https://cratebin.biz/sitemap.xml
```

### 6. Создай контент для SEO

Добавь страницу "About" или "Features":
```
frontend/src/routes/about/+page.svelte
```

С текстом:
- Что такое Cratebin
- Как использовать
- Преимущества перед другими pastebin
- Примеры использования
- FAQ

### 7. Получи обратные ссылки

Добавь проект на:
- Product Hunt
- Hacker News
- Reddit (r/selfhosted, r/opensource)
- Dev.to
- GitHub Awesome Lists
- AlternativeTo.net

### 8. Мониторинг позиций

Проверяй позиции по ключевым словам:
- "free pastebin"
- "code snippet sharing"
- "pastebin alternative"
- "share code online"
- "online code paste"

Инструменты:
- Google Search Console
- Ahrefs (платно)
- SEMrush (платно)
- Ubersuggest (бесплатно)

## Ключевые слова для продвижения

**Основные:**
- pastebin
- code sharing
- snippet sharing
- paste code
- share text online

**Длинные (long-tail):**
- free pastebin alternative
- online code snippet sharing
- share code with password
- temporary text sharing
- self-hosted pastebin

**Конкуренты:**
- pastebin.com alternative
- hastebin alternative
- privatebin alternative
- ghostbin alternative

## Советы

1. **Скорость загрузки** - уже быстро (SvelteKit + Docker)
2. **Mobile-friendly** - уже адаптивный дизайн
3. **HTTPS** - обязательно настрой SSL
4. **Регулярные обновления** - добавляй новые фичи
5. **Блог** - пиши статьи про использование
6. **Социальные сети** - делись в Twitter, Reddit

## Проверка SEO

Инструменты для проверки:
- https://pagespeed.web.dev/ - скорость
- https://search.google.com/test/mobile-friendly - мобильная версия
- https://validator.w3.org/ - валидность HTML
- https://www.seobility.net/en/seocheck/ - общий SEO аудит
