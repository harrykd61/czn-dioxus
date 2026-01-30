# Руководство по юзабилити czn-dioxus

Этот документ описывает принципы и рекомендации по юзабилити приложения czn-dioxus.

## Обзор юзабилити

Наш подход к юзабилити основан на:

- **Простоте** - Интуитивно понятный интерфейс
- **Доступности** - Поддержка пользователей с ограниченными возможностями
- **Комфорте** - Удобное взаимодействие
- **Эффективности** - Быстрое выполнение задач
- **Консистентности** - Единый стиль и поведение

## Принципы дизайна

### 1. Простота и ясность

**Цель**: Минимизация когнитивной нагрузки

**Принципы**:
- **Минимализм** - Только необходимые элементы
- **Иерархия** - Четкая визуальная иерархия
- **Контраст** - Хорошая читаемость
- **Пространство** - Достаточные отступы

**Примеры**:
```css
/* Хорошо: Чистый и понятный интерфейс */
.main-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
    background: #ffffff;
}

/* Плохо: Перегруженный интерфейс */
.main-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 0;
    background: #f0f0f0;
    border: 1px solid #ccc;
    box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}
```

### 2. Последовательность

**Цель**: Предсказуемое поведение интерфейса

**Принципы**:
- **Единый стиль** - Одинаковые элементы ведут себя одинаково
- **Стандартные паттерны** - Использование привычных интерфейсных паттернов
- **Консистентные цвета** - Единая цветовая схема
- **Стандартные иконки** - Использование понятных иконок

**Примеры**:
```css
/* Единые стили кнопок */
.btn-primary {
    background: #007bff;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 4px;
    cursor: pointer;
}

.btn-secondary {
    background: #6c757d;
    color: white;
    border: none;
    padding: 10px 20px;
    border-radius: 4px;
    cursor: pointer;
}

/* Единые стили форм */
.form-group {
    margin-bottom: 15px;
}

.form-control {
    width: 100%;
    padding: 8px 12px;
    border: 1px solid #ccc;
    border-radius: 4px;
    font-size: 14px;
}
```

### 3. Обратная связь

**Цель**: Информирование пользователя о состоянии системы

**Принципы**:
- **Визуальная обратная связь** - Подсветка активных элементов
- **Звуковая обратная связь** - Звуковые сигналы (по желанию)
- **Текстовые сообщения** - Понятные сообщения об ошибках и успехах
- **Прогресс** - Индикаторы выполнения операций

**Примеры**:
```css
/* Визуальная обратная связь */
.btn-primary:hover {
    background: #0056b3;
    transform: translateY(-1px);
}

.btn-primary:active {
    transform: translateY(0);
}

/* Состояния загрузки */
.loading-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(255, 255, 255, 0.8);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
}

/* Сообщения об ошибках */
.alert-error {
    background: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
    padding: 15px;
    border-radius: 4px;
    margin-bottom: 20px;
}

/* Сообщения об успехе */
.alert-success {
    background: #d4edda;
    color: #155724;
    border: 1px solid #c3e6cb;
    padding: 15px;
    border-radius: 4px;
    margin-bottom: 20px;
}
```

## Интерфейс пользователя

### 1. Главное окно

**Структура**:
```
┌─────────────────────────────────────────┐
│              Меню                       │
├─────────────┬───────────────────────────┤
│             │                           │
│   Навигация │        Контент            │
│             │                           │
│             │                           │
└─────────────┴───────────────────────────┘
```

**Рекомендации**:
- **Фиксированное меню** - Всегда доступное меню
- **Боковая навигация** - Быстрый доступ к основным функциям
- **Центральный контент** - Основная рабочая область
- **Статус-бар** - Информация о состоянии системы

### 2. Работа с сертификатами

**Интерфейс списка**:
```html
<div class="certificate-list">
    <div class="certificate-item valid">
        <div class="certificate-info">
            <h4>Сертификат подписи</h4>
            <p>Организация: ООО "Тест"</p>
            <p>Срок действия: до 31.12.2025</p>
        </div>
        <div class="certificate-actions">
            <button class="btn-primary">Использовать</button>
            <button class="btn-secondary">Подробнее</button>
        </div>
    </div>
    
    <div class="certificate-item expired">
        <div class="certificate-info">
            <h4>Просроченный сертификат</h4>
            <p>Организация: ООО "Тест"</p>
            <p>Срок действия: до 31.12.2023</p>
        </div>
        <div class="certificate-actions">
            <button class="btn-disabled" disabled>Недоступен</button>
        </div>
    </div>
</div>
```

**Стили**:
```css
.certificate-list {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 20px;
}

.certificate-item {
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 20px;
    background: #ffffff;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.certificate-item.valid {
    border-left: 4px solid #28a745;
}

.certificate-item.expired {
    border-left: 4px solid #dc3545;
    opacity: 0.7;
}

.certificate-info h4 {
    margin: 0 0 10px 0;
    color: #333;
}

.certificate-info p {
    margin: 5px 0;
    color: #666;
    font-size: 14px;
}
```

### 3. Подпись документов

**Интерфейс выбора**:
```html
<div class="signing-flow">
    <div class="step active">
        <h3>1. Выберите документ</h3>
        <div class="file-dropzone">
            <input type="file" id="file-input" multiple>
            <label for="file-input">
                <span>Перетащите файлы сюда</span>
                <span>или нажмите для выбора</span>
            </label>
        </div>
    </div>
    
    <div class="step">
        <h3>2. Выберите сертификат</h3>
        <div class="certificate-selector">
            <!-- Список доступных сертификатов -->
        </div>
    </div>
    
    <div class="step">
        <h3>3. Подпишите документ</h3>
        <div class="signing-options">
            <button class="btn-primary">Подписать</button>
            <button class="btn-secondary">Просмотреть</button>
        </div>
    </div>
</div>
```

**Стили**:
```css
.signing-flow {
    max-width: 800px;
    margin: 0 auto;
}

.step {
    background: #f8f9fa;
    border-radius: 8px;
    padding: 20px;
    margin-bottom: 20px;
    border-left: 4px solid #dee2e6;
}

.step.active {
    border-left-color: #007bff;
    background: #e9f2ff;
}

.step h3 {
    margin: 0 0 15px 0;
    color: #333;
}

.file-dropzone {
    border: 2px dashed #007bff;
    border-radius: 8px;
    padding: 40px;
    text-align: center;
    background: #ffffff;
    transition: all 0.3s ease;
}

.file-dropzone:hover {
    border-color: #0056b3;
    background: #f8f9ff;
}

.file-dropzone input[type="file"] {
    display: none;
}

.file-dropzone label {
    cursor: pointer;
    display: block;
}

.file-dropzone label span {
    display: block;
    color: #666;
    font-size: 14px;
    margin-bottom: 5px;
}
```

## Доступность

### 1. Клавиатурная навигация

**Требования**:
- **Tab навигация** - Все интерактивные элементы доступны через Tab
- **Горячие клавиши** - Быстрый доступ к основным функциям
- **Фокус** - Видимая индикация фокуса

**Реализация**:
```css
/* Видимый фокус */
*:focus {
    outline: 2px solid #007bff;
    outline-offset: 2px;
}

/* Скрытие фокуса при мышечном вводе */
.js-focus-visible :focus:not(.focus-visible) {
    outline: none;
}

/* Горячие клавиши */
.shortcut-hint {
    font-size: 12px;
    color: #666;
    float: right;
}
```

### 2. Экранная клавиатура

**Требования**:
- **Размер элементов** - Минимальный размер 44x44px
- **Интервалы** - Достаточные промежутки между элементами
- **Контраст** - Хорошая видимость на всех фоновых изображениях

**Реализация**:
```css
/* Минимальный размер элементов */
.interactive-element {
    min-height: 44px;
    min-width: 44px;
    padding: 12px 16px;
    font-size: 16px;
}

/* Интервалы */
.interactive-element + .interactive-element {
    margin-left: 8px;
}
```

### 3. Экранодромы

**Требования**:
- **ARIA метки** - Понятные метки для всех элементов
- **Роль элементов** - Корректные ARIA роли
- **Состояния** - Информация о состоянии элементов

**Реализация**:
```html
<!-- Кнопка с ARIA метками -->
<button 
    aria-label="Подписать документ"
    aria-describedby="sign-help"
    aria-disabled="false"
>
    Подписать
</button>

<div id="sign-help" class="sr-only">
    Нажмите для подписи выбранного документа
</div>

<!-- Скрытие элементов от экранодромов -->
<div class="sr-only">
    Текст только для экранодромов
</div>

<style>
.sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
}
</style>
```

## Адаптивный дизайн

### 1. Мобильные устройства

**Требования**:
- **Сенсорный интерфейс** - Элементы подходят для сенсорного ввода
- **Упрощенный интерфейс** - Оптимизация для маленьких экранов
- **Быстрая загрузка** - Минимизация ресурсов

**Реализация**:
```css
/* Мобильные стили */
@media (max-width: 768px) {
    .main-container {
        padding: 10px;
    }
    
    .certificate-list {
        grid-template-columns: 1fr;
    }
    
    .signing-flow {
        padding: 10px;
    }
    
    .step {
        padding: 15px;
    }
    
    /* Увеличение размеров для сенсорного ввода */
    .btn-primary, .btn-secondary {
        min-height: 48px;
        font-size: 16px;
        padding: 14px 20px;
    }
    
    /* Скрытие ненужных элементов */
    .desktop-only {
        display: none;
    }
}
```

### 2. Планшеты

**Требования**:
- **Средний размер экрана** - Оптимизация для экранов 7-10 дюймов
- **Горизонтальная ориентация** - Поддержка обоих ориентаций

**Реализация**:
```css
/* Планшетные стили */
@media (min-width: 769px) and (max-width: 1024px) {
    .certificate-list {
        grid-template-columns: repeat(2, 1fr);
    }
    
    .signing-flow {
        padding: 20px;
    }
    
    /* Адаптация навигации */
    .sidebar {
        width: 250px;
    }
    
    .content {
        margin-left: 250px;
    }
}
```

### 3. Десктоп

**Требования**:
- **Полноценный интерфейс** - Все функции доступны
- **Многозадачность** - Поддержка нескольких окон
- **Производительность** - Оптимизация для мощных устройств

**Реализация**:
```css
/* Десктопные стили */
@media (min-width: 1025px) {
    .certificate-list {
        grid-template-columns: repeat(3, 1fr);
    }
    
    .signing-flow {
        max-width: 1000px;
    }
    
    /* Полноценная навигация */
    .sidebar {
        width: 300px;
        position: fixed;
        height: 100vh;
    }
    
    .content {
        margin-left: 300px;
        padding: 30px;
    }
}
```

## Интерактивные элементы

### 1. Кнопки

**Типы кнопок**:
```css
/* Первичная кнопка */
.btn-primary {
    background: #007bff;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 6px;
    font-size: 16px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
    box-shadow: 0 2px 4px rgba(0, 123, 255, 0.2);
}

.btn-primary:hover {
    background: #0056b3;
    transform: translateY(-1px);
    box-shadow: 0 4px 8px rgba(0, 123, 255, 0.3);
}

.btn-primary:active {
    transform: translateY(0);
    box-shadow: 0 1px 2px rgba(0, 123, 255, 0.2);
}

/* Вторичная кнопка */
.btn-secondary {
    background: #6c757d;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 6px;
    font-size: 16px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

/* Опасная кнопка */
.btn-danger {
    background: #dc3545;
    color: white;
    border: none;
    padding: 12px 24px;
    border-radius: 6px;
    font-size: 16px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease;
}

/* Отключенная кнопка */
.btn-disabled {
    background: #e9ecef;
    color: #6c757d;
    border: none;
    padding: 12px 24px;
    border-radius: 6px;
    font-size: 16px;
    font-weight: 500;
    cursor: not-allowed;
    opacity: 0.6;
}
```

### 2. Формы

**Типы полей**:
```css
/* Текстовые поля */
.form-control {
    width: 100%;
    padding: 12px 16px;
    border: 2px solid #e9ecef;
    border-radius: 6px;
    font-size: 16px;
    transition: border-color 0.2s ease;
    background: #ffffff;
}

.form-control:focus {
    outline: none;
    border-color: #007bff;
    box-shadow: 0 0 0 3px rgba(0, 123, 255, 0.1);
}

.form-control.error {
    border-color: #dc3545;
    box-shadow: 0 0 0 3px rgba(220, 53, 69, 0.1);
}

/* Выпадающие списки */
.form-select {
    width: 100%;
    padding: 12px 16px;
    border: 2px solid #e9ecef;
    border-radius: 6px;
    font-size: 16px;
    background: #ffffff;
    cursor: pointer;
    appearance: none;
    background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 20 20'%3e%3cpath fill='none' stroke='%23666' stroke-linecap='round' stroke-linejoin='round' stroke-width='2' d='m6 8l4 4 4-4'/%3e%3c/svg%3e");
    background-repeat: no-repeat;
    background-position: right 16px center;
    background-size: 12px;
}

/* Чекбоксы и радио */
.form-checkbox, .form-radio {
    width: 18px;
    height: 18px;
    cursor: pointer;
}

/* Лейблы */
.form-label {
    display: block;
    margin-bottom: 8px;
    font-weight: 500;
    color: #333;
}

.form-label.required::after {
    content: " *";
    color: #dc3545;
}
```

### 3. Модальные окна

**Реализация**:
```css
/* Модальное окно */
.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 1000;
    opacity: 0;
    visibility: hidden;
    transition: all 0.3s ease;
}

.modal-overlay.open {
    opacity: 1;
    visibility: visible;
}

.modal-content {
    background: #ffffff;
    border-radius: 8px;
    box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
    max-width: 500px;
    width: 90%;
    max-height: 80vh;
    overflow-y: auto;
    transform: translateY(20px);
    transition: transform 0.3s ease;
}

.modal-overlay.open .modal-content {
    transform: translateY(0);
}

.modal-header {
    padding: 20px;
    border-bottom: 1px solid #eee;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

.modal-title {
    margin: 0;
    font-size: 20px;
    color: #333;
}

.modal-close {
    background: none;
    border: none;
    font-size: 24px;
    cursor: pointer;
    color: #666;
    padding: 0;
    width: 30px;
    height: 30px;
    display: flex;
    align-items: center;
    justify-content: center;
    border-radius: 50%;
}

.modal-close:hover {
    background: #f8f9fa;
    color: #333;
}

.modal-body {
    padding: 20px;
}

.modal-footer {
    padding: 20px;
    border-top: 1px solid #eee;
    display: flex;
    justify-content: flex-end;
    gap: 10px;
}
```

## Анимации и переходы

### 1. Микровзаимодействия

**Цель**: Улучшение пользовательского опыта

**Примеры**:
```css
/* Плавное появление */
.fade-in {
    animation: fadeIn 0.3s ease-in;
}

@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(10px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}

/* Плавное исчезновение */
.fade-out {
    animation: fadeOut 0.3s ease-out;
}

@keyframes fadeOut {
    from {
        opacity: 1;
        transform: translateY(0);
    }
    to {
        opacity: 0;
        transform: translateY(10px);
    }
}

/* Плавное изменение размера */
.scale-in {
    animation: scaleIn 0.2s ease-out;
}

@keyframes scaleIn {
    from {
        transform: scale(0.9);
        opacity: 0;
    }
    to {
        transform: scale(1);
        opacity: 1;
    }
}

/* Плавное изменение цвета */
.color-transition {
    transition: background-color 0.3s ease, color 0.3s ease;
}
```

### 2. Загрузка и прогресс

**Индикаторы загрузки**:
```css
/* Спиннер */
.spinner {
    width: 40px;
    height: 40px;
    border: 4px solid #f3f3f3;
    border-top: 4px solid #007bff;
    border-radius: 50%;
    animation: spin 1s linear infinite;
}

@keyframes spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}

/* Прогресс-бар */
.progress-bar {
    width: 100%;
    height: 8px;
    background: #f0f0f0;
    border-radius: 4px;
    overflow: hidden;
}

.progress-fill {
    height: 100%;
    background: #007bff;
    width: 0%;
    transition: width 0.3s ease;
}

/* Круговой прогресс */
.progress-circle {
    width: 60px;
    height: 60px;
    position: relative;
}

.progress-circle svg {
    transform: rotate(-90deg);
}

.progress-circle circle {
    fill: none;
    stroke-width: 4;
    stroke-linecap: round;
    transition: stroke-dashoffset 0.3s ease;
}
```

## Best Practices

### 1. Тестирование юзабилити

- **A/B тестирование** - Сравнение разных вариантов интерфейса
- **Юзабилити тесты** - Тестирование с реальными пользователями
- **Анализ поведения** - Изучение поведения пользователей
- **Обратная связь** - Сбор отзывов и предложений

### 2. Доступность

- **WCAG 2.1** - Соблюдение стандартов доступности
- **Тестирование** - Регулярное тестирование с экранодромами
- **Обучение** - Обучение команды принципам доступности
- **Инструменты** - Использование инструментов для проверки доступности

### 3. Производительность

- **Быстрая загрузка** - Минимизация времени загрузки
- **Плавная анимация** - 60 FPS для всех анимаций
- **Оптимизация** - Оптимизация изображений и кода
- **Кэширование** - Эффективное кэширование ресурсов

## Поддержка юзабилити

### 1. Документация

- **Style Guide** - Руководство по стилям
- **Component Library** - Библиотека компонентов
- **UX Guidelines** - Руководство по юзабилити
- **Accessibility Guide** - Руководство по доступности

### 2. Инструменты

- **Figma** - Дизайн и прототипирование
- **Lighthouse** - Анализ доступности и производительности
- **axe** - Инструмент для проверки доступности
- **UserTesting** - Платформа для юзабилити тестирования

### 3. Контакты

- **UX Team**: ux@czn-dioxus.com
- **Design Team**: design@czn-dioxus.com
- **Accessibility Team**: accessibility@czn-dioxus.com

## Обновления

Последнее обновление: 2024-01-01

Для получения актуальной информации:
- [Nielsen Norman Group](https://www.nngroup.com/)
- [WCAG Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [Material Design](https://material.io/design)
- [Apple Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/)
