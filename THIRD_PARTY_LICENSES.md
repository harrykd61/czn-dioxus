# Сторонние лицензии czn-dioxus

Этот документ содержит информацию о лицензиях сторонних компонентов, используемых в приложении czn-dioxus.

## Обзор лицензий

Наш подход к лицензированию сторонних компонентов основан на:

- **Соответствии** - Соответствие лицензий требованиям проекта
- **Прозрачности** - Полная прозрачность использования сторонних компонентов
- **Совместимости** - Совместимость лицензий с нашей лицензией
- **Обновлении** - Регулярное обновление информации о лицензиях

## Зависимости Rust

### 1. Основные зависимости

#### axum
- **Версия**: 0.7.3
- **Лицензия**: MIT OR Apache-2.0
- **Назначение**: Веб-фреймворк
- **URL**: https://github.com/tokio-rs/axum

#### tokio
- **Версия**: 1.35.1
- **Лицензия**: MIT OR Apache-2.0
- **Назначение**: Асинхронная среда выполнения
- **URL**: https://github.com/tokio-rs/tokio

#### serde
- **Версия**: 1.0.193
- **Лицензия**: Apache-2.0 OR MIT
- **Назначение**: Сериализация/десериализация
- **URL**: https://github.com/serde-rs/serde

#### tracing
- **Версия**: 0.1.40
- **Лицензия**: Apache-2.0 OR MIT
- **Назначение**: Логирование и трассировка
- **URL**: https://github.com/tokio-rs/tracing

#### reqwest
- **Версия**: 0.11.19
- **Лицензия**: MIT OR Apache-2.0
- **Назначение**: HTTP клиент
- **URL**: https://github.com/seanmonstar/reqwest

### 2. Криптографические зависимости

#### ring
- **Версия**: 0.17.8
- **Лицензия**: BSD-3-Clause
- **Назначение**: Криптографические операции
- **URL**: https://github.com/briansmith/ring

#### openssl
- **Версия**: 0.10.57
- **Лицензия**: Apache-2.0
- **Назначение**: OpenSSL bindings
- **URL**: https://github.com/sfackler/rust-openssl

#### base64
- **Версия**: 0.21.5
- **Лицензия**: MIT OR Apache-2.0
- **Назначение**: Base64 кодирование/декодирование
- **URL**: https://github.com/marshallpierce/rust-base64

### 3. База данных

#### sqlx
- **Версия**: 0.7.2
- **Лицензия**: Apache-2.0
- **Назначение**: Асинхронный SQL фреймворк
- **URL**: https://github.com/launchbadge/sqlx

#### postgres
- **Версия**: 0.7.8
- **Лицензия**: Apache-2.0
- **Назначение**: PostgreSQL драйвер
- **URL**: https://github.com/sfackler/rust-postgres

### 4. Системные зависимости

#### sysinfo
- **Версия**: 0.29.4
- **Лицензия**: MIT OR Apache-2.0
- **Назначение**: Системная информация
- **URL**: https://github.com/GuillaumeGomez/sysinfo

#### chrono
- **Версия**: 0.4.26
- **Лицензия**: MIT OR Apache-2.0
- **Назначение**: Работа с датами и временем
- **URL**: https://github.com/chronotope/chrono

### 5. Утилиты

#### clap
- **Версия**: 4.4.12
- **Лицензия**: MIT OR Apache-2.0
- **Назначение**: CLI аргументы
- **URL**: https://github.com/clap-rs/clap

#### anyhow
- **Версия**: 1.0.75
- **Лицензия**: MIT OR Apache-2.0
- **Назначение**: Обработка ошибок
- **URL**: https://github.com/dtolnay/anyhow

#### thiserror
- **Версия**: 1.0.50
- **Лицензия**: MIT OR Apache-2.0
- **Назначение**: Определение ошибок
- **URL**: https://github.com/dtolnay/thiserror

## Зависимости JavaScript/TypeScript

### 1. Frontend зависимости

#### react
- **Версия**: 18.2.0
- **Лицензия**: MIT
- **Назначение**: UI фреймворк
- **URL**: https://github.com/facebook/react

#### vite
- **Версия**: 5.0.11
- **Лицензия**: MIT
- **Назначение**: Build система
- **URL**: https://github.com/vitejs/vite

#### tailwindcss
- **Версия**: 3.3.6
- **Лицензия**: MIT
- **Назначение**: CSS фреймворк
- **URL**: https://github.com/tailwindlabs/tailwindcss

#### dioxus
- **Версия**: 0.4.0
- **Лицензия**: MIT
- **Назначение**: Frontend фреймворк
- **URL**: https://github.com/DioxusLabs/dioxus

### 2. UI компоненты

#### @headlessui/react
- **Версия**: 1.7.17
- **Лицензия**: MIT
- **Назначение**: UI компоненты
- **URL**: https://github.com/tailwindlabs/headlessui

#### @heroicons/react
- **Версия**: 2.0.18
- **Лицензия**: MIT
- **Назначение**: Иконки
- **URL**: https://github.com/tailwindlabs/heroicons

### 3. Утилиты

#### clsx
- **Версия**: 2.0.0
- **Лицензия**: MIT
- **Назначение**: Условные CSS классы
- **URL**: https://github.com/lukeed/clsx

#### zod
- **Версия**: 3.22.4
- **Лицензия**: MIT
- **Назначение**: Валидация данных
- **URL**: https://github.com/colinhacks/zod

## Docker образы

### 1. Официальные образы

#### postgres:15
- **Лицензия**: PostgreSQL License
- **Назначение**: База данных
- **URL**: https://www.postgresql.org/about/licence/

#### redis:7-alpine
- **Лицензия**: BSD 3-clause "New" or "Revised" License
- **Назначение**: Кэширование
- **URL**: https://redis.io/topics/license

#### nginx:alpine
- **Лицензия**: 2-clause BSD-like license
- **Назначение**: Веб-сервер
- **URL**: https://nginx.org/LICENSE

### 2. Инструменты мониторинга

#### prom/prometheus
- **Лицензия**: Apache-2.0
- **Назначение**: Мониторинг
- **URL**: https://github.com/prometheus/prometheus

#### grafana/grafana
- **Лицензия**: AGPL-3.0
- **Назначение**: Визуализация метрик
- **URL**: https://github.com/grafana/grafana

#### elasticsearch:8.11.0
- **Лицензия**: Elastic License
- **Назначение**: Поисковый движок
- **URL**: https://www.elastic.co/licensing/elastic-license

## Лицензионные требования

### 1. MIT License

**Требования**:
- Сохранение уведомления об авторских правах
- Сохранение лицензионного текста
- Отсутствие ограничений на использование

**Пример уведомления**:
```text
Copyright (c) [год] [владелец авторских прав]

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

### 2. Apache License 2.0

**Требования**:
- Сохранение уведомления об авторских правах
- Сохранение лицензионного текста
- Уведомление о изменениях
- Сохранение NOTICE файлов

**Пример уведомления**:
```text
Copyright [год] [владелец авторских прав]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

### 3. BSD License

**Требования**:
- Сохранение уведомления об авторских правах
- Сохранение лицензионного текста
- Ограничение использования имени для рекламы

**Пример уведомления**:
```text
Copyright (c) [год], [владелец авторских прав]
All rights reserved.

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this
   list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
   contributors may be used to endorse or promote products derived from
   this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
```

## Совместимость лицензий

### 1. Анализ совместимости

**MIT License**:
- Совместим с Apache-2.0
- Совместим с BSD
- Совместим с большинством лицензий

**Apache License 2.0**:
- Совместим с MIT
- Совместим с BSD
- Требует уведомления о патентных правах

**BSD License**:
- Совместим с MIT
- Совместим с Apache-2.0
- Ограниченное использование имени

### 2. Конфликты лицензий

**AGPL-3.0**:
- Не совместим с MIT в некоторых случаях
- Требует раскрытия исходного кода
- Ограничения на коммерческое использование

**Elastic License**:
- Не является OSS лицензией
- Ограничения на коммерческое использование
- Требует лицензирования

## Управление лицензиями

### 1. Автоматический сбор лицензий

```bash
# Для Rust зависимостей
cargo-license --format yaml > licenses-rust.yaml

# Для JavaScript зависимостей
license-checker --json > licenses-js.json

# Для Docker образов
docker run --rm -v /var/run/docker.sock:/var/run/docker.sock \
  containrrr/license-checker \
  --image postgres:15 \
  --image redis:7-alpine
```

### 2. Проверка лицензий

```bash
# Проверка совместимости лицензий
cargo deny check licenses

# Проверка наличия лицензий
cargo-license --summary

# Генерация отчета
cargo-license --format markdown > THIRD-PARTY-LICENSES.md
```

### 3. Обновление лицензий

**Процесс обновления**:
1. **Анализ изменений** - Проверка изменений в зависимостях
2. **Проверка лицензий** - Проверка новых лицензий
3. **Оценка совместимости** - Оценка совместимости с нашей лицензией
4. **Обновление документации** - Обновление документов
5. **Уведомление** - Уведомление команды о изменениях

## Best Practices

### 1. Выбор зависимостей

- **Проверка лицензий** - Проверка лицензий перед использованием
- **Совместимость** - Проверка совместимости с нашей лицензией
- **Поддержка** - Выбор активно поддерживаемых зависимостей
- **Безопасность** - Проверка на уязвимости

### 2. Документирование

- **Полная информация** - Полная информация о всех зависимостях
- **Актуальность** - Регулярное обновление информации
- **Прозрачность** - Прозрачность использования сторонних компонентов
- **Доступность** - Доступность информации для всех заинтересованных

### 3. Соблюдение лицензий

- **Следование требованиям** - Следование всем требованиям лицензий
- **Уведомления** - Правильное размещение уведомлений
- **Атрибуция** - Правильная атрибуция авторов
- **Контроль** - Регулярный контроль соблюдения лицензий

## Поддержка лицензирования

### 1. Документация

- **License Guidelines** - Руководство по лицензированию
- **Compatibility Matrix** - Матрица совместимости лицензий
- **Best Practices** - Лучшие практики
- **FAQ** - Часто задаваемые вопросы

### 2. Инструменты

- **License Checker** - Инструменты проверки лицензий
- **Automated Tools** - Автоматизированные инструменты
- **Monitoring** - Мониторинг изменений лицензий
- **Reporting** - Генерация отчетов

### 3. Контакты

- **Legal Team**: legal@czn-dioxus.com
- **Compliance Team**: compliance@czn-dioxus.com
- **Open Source Team**: oss@czn-dioxus.com

## Обновления

Последнее обновление: 2024-01-01

**Частота обновления**: Ежеквартально

**Процесс обновления**:
1. Автоматический сбор лицензий
2. Проверка изменений
3. Обновление документации
4. Уведомление команды

Для получения актуальной информации:
- [SPDX License List](https://spdx.org/licenses/)
- [Choose a License](https://choosealicense.com/)
- [FOSSA License Compliance](https://fossa.com/product/license-compliance/)
- [OSS Review Toolkit](https://oss-review-toolkit.org/)
