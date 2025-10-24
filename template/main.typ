#set page(width: 72mm, height: auto, margin: 0pt)

#set text(
  font: "PP Mondwest",
  size: 11pt,
  region: "GB"
)

#show heading: text.with(size: 2em)
#show raw: text.with(font: "Source Code Pro")

#let reference = "{{reference}}"
#let name = "{{name}}"
#let discord = "{{discord}}"
#let pizza = "{{pizza}}"
#let release_title = "{{release_title}}"

#[
  #set align(center)
  = #release_title

  #image("castle.png", width: 80%)
]

NAME#h(1fr)#name \
REFERENCE#h(1fr)#reference \
LUNCH 1#h(1fr)\[ \] \
LUNCH 2#h(1fr)\[ \]

#pagebreak()

#[
  #set align(center)
  = PIZZA VOUCHER

  #name
  ```
   ____                   
  /    \			
    u  u|      _______    
      \ |  .-''#%&#&%#``-.   
     = /  ((%&#&#&%&VK&%&))  
      |    `-._#%&##&%_.-'   
   /\/\`--.   `-."".-'
   |  |    \   /`./          
   |\/|  \  `-'  /
   || |   \     /            
  ```
  #[
    #set text(size: 16.5pt)
    #pizza
  ]
]

