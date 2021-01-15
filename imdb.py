#to get rating information 
#https://www.imdb.com/user/ur57221798/ratings/export
import urllib.request

webUrl = urllib.request.urlopen('https://www.imdb.com/user/ur57221798/ratings/export')
print(webUrl.getcode()))
