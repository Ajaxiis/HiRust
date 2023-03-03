# Interactive-Graphical-Rust-Aplication-builder-

ru-Ru:
<h3>Эта версия -не рабочая, фукционал в процессе написания </h3>
Добро пожаловать в Конструктор программ на языке Раст!
ИГРА - Интерактивный Графический Раст Апликаций (конструктор).
Концепция конструктора была разработана под влиянием проекта Hiasm (http:// hiasm.com)  
Назначением данного проекта является быстрое прототипирование апликаций  на Rust для всех существующих операционных систем и WEB. а так же популяризация языка программирования Rust.
Парадигма комбинации графического (алгоритмического) построения программ является наиболее прогрессивной и быстрой на данный момент.
Это удобно как новичкам, так и профессионалам (для быстрого прототипирования). 
К сожалению, за наглядность алгоритма приходится платить ужасным с точки зрения профессионалов кодом. 
Но быстро (в течение нескольких минут) набросать ( нарисовать) прототип программы да еще и с графическим интерфейсом с помощью мыши или пальцем на планшете, и показать его на любом устройстве - согласитесь, это КРУТО!
<p>Преимущества :
1) Готовая скомпилированаая программа под любую платформу, имеющая малый вес.<br>
2) Быстрая разработка на основе готовых шаблонов.<br>
3) Не обязательное знание языков программирования для работы (подходит новичкам и людям с дисграфией)<br>
4) Ни строчки кода при работе. <br>
5) Умный компилятор подгрузит необходимое во время работы и сообщит об ошибках.
</p>
<p>
Недостатки:
1) Плохочитаемый, неоптимизированный код исходника (файл main.rs)<br>
2) Потребление большого количества памяти и памяти видео, потребление батареи. <br>
3) необходимость подключения к интернету для обновлений и при компиляции. <br>
</p>
<i>Нам очень нужна Ваша помощь и поддержка!<br>
Если Вы знаете Раст, вы можете помочь проекту своими советами, кодом и делом :)<br>
Если Вы не знаете Раст но хотите помочь, то вы можете принять участие в переводах, уроках, статьях, рекламе и т.п.</i>

Огромная благодарность сообществу Rust и авторам языка.
Огромная благодарность Dilma  и команде разработчиков hiasm (<a blanc_>http://hiasm.com</a>) .
Преогромная благодарность Emil Ernerfeldt за билиотеку egui!(<a blanc_>https://www.egui.rs/#demo</a>)

КАК ЭТО РАБОТАЕТ.
Основная концепция конструтора базируется на создании элементов как "шаблонов" в отдельных файлах .toml из готовых библиотек  или производных из них.
Например :
Раздел элементов для графического интерфейса базируется на библиотеке egui (или другой) в данном случае библиотека становится "пакетом" и используя ее API 
создаются элементы окон, кнопок, полей и т.п. как отдельные файлы для каждого элемента со строгой типизацией и шаблоном кода Rust.

<h3>Алгоритм абстракции:</h3> 
Абстрактный блок ( часть программы) хранится в нескольких файлах: 
element.png - иконка элемента, element.toml - данные  элемента. 
 Содержание файла element.toml:
<br>Название элемента - переменная для схемы.
<br>Версия- номер версии элемента.
<br>Иконка - соответствующая иконка для схемы. 
<br>>Категория - место иконки в соответствующем разделе графического интерфейса. 
<br>Пакет- библиотека содержащая данный элемент из сайта crates.io для компилятора (прописывается в файле cargo.toml) компилируемого проекта. 
<br>Методы - вектор перечисления точек методов. 
<br>Свойства методов - вектор соответствующий методам (описание переменных)
<br>Значения: вектор значений по умолчанию для методов.
<br>Функции- вектор кодов команд соответствующих методов. 
<br>Переменные: список переменных для элемента 
<br>Свойства переменных: соответствующий вектор переменным с описанием 
<br>Значения переменных : соответствующий вектор переменных со значением по умолчанию (если нужно) 
</br>
<h3>Как это выглядит на схеме:</h3> 
Элемент- это иконка ( квадрат или прямоугольник) на котором слева точки входа ( функции) справа точки выхода  потока данных или команд. Сверху - точки используемых переменных, снизу - значения переменных в элементе. 

Например: элемент if будет содержать точки : 
Слева :doif- точка запуска элемента , 
сверху :2 точки сравниваемых переменных , 
справа - 2 точки : ок- при выполнении условия и else если условие не выполняется. 

Работа программы: 
Линкер - это служебная функция, назначение которой перевести графическую абстракцию готовой схемы в служебный код для компилятора. 
Поскольку мы будем использовать внешний линкер cargo для компиляции,  то компиляция схемы в код будет состоять из трех этапов:
<li>Линковка - преобразование графической схемы в код.</li>
<li>Кодировка - создание и сохранение конечного кода для компилятора.</li>
<li>Запуск компилятора cargo для построения бинарного выходного кода или библиотеки . 
Идея: продумать возможность загрузки скомпилированного на crates.io или  github.com
 
<h3>Работа линкера:</h3>
При соединении точек в схеме, линкер обращается к файлам соединяемых компонентов. При этом происходит "оптимизация" создание внутренних переменных и наполнение их ссылок на файлы используемых элементов схемы.
Очевидно, что не нужно каждый раз обращаться к одному и тому же файлу компонента если этот компонент используется множество раз.
Достаточно пропарсить файл единожды и сохранить для каждого последующего компонента значение отличающееся от предыдущих.
Таким образом создаётся типа дорожной карты для  парсера -кодогенератора.<br>
<strong>Алгоритм работы линкера: </strong>
Парсим файл схемы на наличие элементов, создаём ссылки на файлы элементов в виде списка для парсинга.
