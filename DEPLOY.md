# Деплой на VPS (cratebin.biz)

## Быстрый старт

```bash
# 1. Подключись к VPS
ssh root@your-vps-ip

# 2. Установи Docker (если нет)
curl -fsSL https://get.docker.com -o get-docker.sh
sh get-docker.sh

# 3. Клонируй проект
d

# 4. Скопируй .env файлы (они уже настроены для cratebin.biz)
cp backend/.env.example backend/.env
cp frontend/.env.example frontend/.env

# 5. Запусти
docker-compose up -d --build

# 6. Проверь
docker-compose ps
docker-compose logs -f
```

## Настройка DNS

Добавь A-запись в DNS:
```
cratebin.biz  →  IP твоего VPS
```

## Настройка HTTPS с Nginx

### 1. Установи Nginx и Certbot
```bash
apt update
apt install -y nginx certbot python3-certbot-nginx
```

### 2. Создай конфиг Nginx
```bash
nano /etc/nginx/sites-available/cratebin
```

Вставь:
```nginx
server {
    listen 80;
    server_name cratebin.biz www.cratebin.biz;

    # Frontend
    location / {
        proxy_pass http://127.0.0.1:3232;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;
    }

    # Backend API
    location /api {
        proxy_pass http://127.0.0.1:8080;
        proxy_http_version 1.1;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### 3. Активируй конфиг
```bash
ln -s /etc/nginx/sites-available/cratebin /etc/nginx/sites-enabled/
rm /etc/nginx/sites-enabled/default  # Удали дефолтный
nginx -t
systemctl restart nginx
```

### 4. Получи SSL сертификат
```bash
certbot --nginx -d cratebin.biz -d www.cratebin.biz
```

Выбери опцию 2 (redirect HTTP to HTTPS)

### 5. Готово!
Открой https://cratebin.biz

## Обновление

```bash
cd Cratebin
git pull
docker-compose up -d --build
```

## Полезные команды

```bash
# Логи
docker-compose logs -f

# Рестарт
docker-compose restart

# Остановка
docker-compose down

# Бэкап БД
docker-compose exec db pg_dump -U cratebin cratebin > backup.sql

# Восстановление БД
docker-compose exec -T db psql -U cratebin cratebin < backup.sql
```

## Troubleshooting

### Порт 80 занят
```bash
sudo lsof -i :80
sudo systemctl stop apache2  # если Apache
```

### Не хватает памяти
```bash
# Добавь swap
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab
```

### Docker не запускается
```bash
sudo systemctl start docker
sudo systemctl enable docker
```
