$(document).ready(function(){
    $('#register-team').click(function(){
        //get query parm
        var tournament_id = getParameterByName('tournament_id');
        location.href="/registerTeam?tournament_id="+tournament_id;
    });
});