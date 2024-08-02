1
PS C:\rust\projects\marketplace\31072024\heads\heads>    https://tailwindcss.com/docs/installation
https://tailwindcss.com/docs/install
ation : Имя "https://tailwindcss.com
/docs/installation" не распознано ка
к имя командлета, функции, файла сце
нария или выполняемой программы. Про
верьте правильность написания имени,
 а также наличие и правильность пути
, после чего повторите попытку.
строка:1 знак:1
+ https://tailwindcss.com/docs/insta
llation
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
~~~~~~~
    + CategoryInfo          : Objec 
   tNotFound: (https://tailwin...c  
  s/installation:String) [], Comm   
 andNotFoundException
    + FullyQualifiedErrorId : Comma 
   ndNotFoundException
 
PS C:\rust\projects\marketplace\31072024\heads\heads> 

2
@tailwind base;
@tailwind components;
@tailwind utilities;
правильные директивы, однако tailwind почему подчеркивается и при наведении кто-то там за кадром говорит 
Unknown at rule @tailwindcss(unknownAtRules)

3
автоисправлением в редакторе кода исправил твой код так 
module.exports = {
  plugins: {
    tailwinds: {}, 
    autoprefixer: {},
  },
};

4
npm run watch:css - это я использую в одном терминале для node для tailwind и он запускается, однако почему возникают все эти подчеркивания, например, в директивах, непонятно - а во втором терминале я запускаю cargo leptos watch и все работает - только я бы стили на более современные поменял - серый космос строго заданный для всех фон, ммм возможно тебе нужно прочитать скинуть пару книг по tailwind и leptos нужно - чтобы компетентнее со мной взаимодействовать? 




отчитываюсь
все сделал и вот что выдало 
PS C:\rust\projects\marketplace\31072024\heads\heads> npm install tailwindcss postcss autoprefixer

up to date, audited 123 packages in 2s

34 packages are looking for funding
  run `npm fund` for details

found 0 vulnerabilities
PS C:\rust\projects\marketplace\31072024\heads\heads> npm run watch:css

> heads@1.0.0 watch:css
> tailwindcss -i src/input.css -o ./style/output.css --watch

Specified input file src/input.css does not exist.
PS C:\rust\projects\marketplace\31072024\heads\heads> 
 *  Журнал восстановлен 

PS C:\rust\projects\marketplace\31072024\heads\heads> 

также установил Tailwind CSS IntelliSense и PostCSS Language Support
отличный совет
