training-cli
============

Futura (forse) interfaccia per [training.olinfo.it](training.olinfo.it) ([cmsocial](https://github.com/algorithm-ninja/cmsocial)).

Installazione
-------------

Copia la repository in una directory locale.
Crea un file in cui conservare il token dell'API di training e inserisci il percorso completo di quel file nella costante `TOKEN_FILE` in `main.rs`.

Esegui `cargo install --path .` dalla directory in cui hai copiato la repository.

Utilizzo
--------

* `training-cli login` esegue interattivamente il login e salva il token nel file specificato.
* `training-cli logout` elimina il file con il token.
* `training-cli submit [task_name] [file1] ...` sottomette i file indicati al task indicato.
* `training-cli list-sub [task-name] [optional: # of subs]` elenca le sottoposizioni più recenti sul task indicato.
* `training-cli sub-details [sub_id]` mostra i dettagli della sottoposizione indicata.
