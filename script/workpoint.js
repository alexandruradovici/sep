function Timer (seconds, music) {
    this.seconds = seconds;
    this.audio = new Audio (music);
    this.audio.loop = true;
    this.audio.preload = true;
    this.value = seconds;
    this.timer = null;
    this.listeners = [];
}

Timer.prototype.start = function () {
    this.value = this.seconds;
    clearInterval (this.timer);
    this.timer = setInterval ((function () {
        if (this.value > 0)
        {
            this.value = this.value - 1;
            this.emit ('value', this.value);
        }
        else 
        {
            clearInterval (this.timer);
        }
    }).bind(this), 1000);
    this.audio.play ();
}

Timer.prototype.stop = function () {
    clearInterval (this.timer);
    this.audio.pause ();
}

Timer.prototype.on = function (event, fn) {
    this.listeners.push (fn);
}

Timer.prototype.emit = function (event, ...data) {
    for (let fn of this.listeners) {
        fn.apply (null, data);
    }
}

Timer.prototype.getValue = function () {
    return this.value;
}

document.addEventListener('DOMContentLoaded', function() {
    function workpoints ()
    {

        let elements = document.querySelectorAll('.workpoint');

        console.log (elements);

        for (let element of elements) {
            
            var div = document.createElement('span');
            div.setAttribute('class', 'workpoint-timer');
            element.append (div);

            function setTime (seconds) {
                let minutes = Math.floor (seconds / 60);
                if (minutes < 10) minutes = '0'+minutes;

                let s = Math.floor (seconds % 60);
                if (s < 10) s = '0'+s;
                div.textContent = minutes+':'+s+' '; 
            }

            var start = document.createElement('i');
            start.setAttribute('class', 'fa fa-play');
            // start.textContent = 'Start';
            start.onclick = function () {
                timer.start ();
                stop.setAttribute('style', '');
                start.setAttribute('style', 'display: none;');
            }
            element.append (start);

            var stop = document.createElement('i');
            stop.setAttribute('class', 'fa fa-pause');
            stop.setAttribute('style', 'display: none;');
            // stop.textContent = 'stop';
            stop.onclick = function () {
                timer.stop ();
                stop.setAttribute('style', 'display: none;');
                start.setAttribute('style', '');
            }
            element.append (stop);

            let seconds = element.getAttribute('seconds');

            setTime (seconds);

            let timer = new Timer (seconds, element.getAttribute('music'));
            timer.on ('value', function (seconds) {
               setTime (seconds);
               if (seconds == 0) {
                   timer.stop();
               }
            });
        }
    }

    workpoints ();
});
