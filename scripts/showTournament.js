$(document).ready(function(){
    $('#register-team').click(function(){
        //get query parm
        var tournament_id = getParameterByName('tourney_id');
        location.href="/registerTeam?tourney_id="+tournament_id;
    });
});