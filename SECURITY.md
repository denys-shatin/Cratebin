# Безопасность

## На VPS сделай:

### 1. Создай сильный пароль для БД

```bash
cd ~/Cratebin

# Создай .env файл
nano .env
```

Добавь:
```env
DB_PASSWORD=ваш_очень_сложный_пароль_123!@#
```

Сохрани (Ctrl+O, Enter, Ctrl+X)

### 2. Обнови docker-compose.yml

Файл уже обновлен в репозитории. Запулли изменения:

```bash
git pull
```

Или вручную измени:
- Убери `ports: - "5432:5432"` из секции `db` (БД не должна быть доступна снаружи!)
- Измени `CORS_ORIGINS` на `https://cratebin.biz`
- Измени порт frontend на `80:3000`

### 3. Перезапусти с новым паролем

```bash
# Останови всё
docker compose down

# Удали старую БД (ВНИМАНИЕ: удалит все данные!)
docker volume rm cratebin_postgres_data

# Запусти с новым паролем
docker compose up -d --build
```

### 4. Настрой firewall

```bash
# Установи ufw
sudo apt install -y ufw

# Разрешь только нужные порты
sudo ufw allow 22/tcp    # SSH
sudo ufw allow 80/tcp    # HTTP
sudo ufw allow 443/tcp   # HTTPS

# Включи firewall
sudo ufw enable

# Проверь статус
sudo ufw status
```

### 5. Настрой fail2ban (защита от брутфорса)

```bash
sudo apt install -y fail2ban
sudo systemctl enable fail2ban
sudo systemctl start fail2ban
```

### 6. Обнови систему

```bash
sudo apt update
sudo apt upgrade -y
```

### 7. Настрой автообновления

```bash
sudo apt install -y unattended-upgrades
sudo dpkg-reconfigure -plow unattended-upgrades
```

## Что исправлено:

✅ **БД закрыта** - порт 5432 не открыт наружу
✅ **Пароль из переменной** - можно задать сильный пароль
✅ **Backend только на localhost** - доступен только через Nginx
✅ **CORS настроен** - только для cratebin.biz
✅ **Frontend на порту 80** - стандартный HTTP порт

## Дополнительно:

### Регулярные бэкапы БД

```bash
# Создай скрипт бэкапа
nano ~/backup.sh
```

Добавь:
```bash
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
docker compose -f ~/Cratebin/docker-compose.yml exec -T db pg_dump -U cratebin cratebin > ~/backups/cratebin_$DATE.sql
# Удаляй старые бэкапы (старше 7 дней)
find ~/backups -name "cratebin_*.sql" -mtime +7 -delete
```

Сделай исполняемым:
```bash
chmod +x ~/backup.sh
mkdir ~/backups
```

Добавь в cron (каждый день в 3 ночи):
```bash
crontab -e
```

Добавь строку:
```
0 3 * * * /home/ubuntu/backup.sh
```

### Мониторинг логов

```bash
# Смотри логи в реальном времени
docker compose logs -f

# Только ошибки
docker compose logs | grep ERROR

# Логи конкретного сервиса
docker compose logs backend
```

### Ограничение ресурсов

Добавь в docker-compose.yml для каждого сервиса:

```yaml
deploy:
  resources:
    limits:
      cpus: '0.5'
      memory: 512M
```

## Проверка безопасности

```bash
# Проверь открытые порты
sudo netstat -tulpn

# Должны быть открыты только: 22, 80, 443

# Проверь Docker сеть
docker network inspect cratebin_default

# БД должна быть видна только внутри сети
```
