$(document).ready(function(){
    $('#register-team').click(function(){
        var teams = $('.team');
        if(teams.length >= 4){
            alert("Only 4 teams allowed per tournament");
        }else{
            //get query parm
            var tournament_id = getParameterByName('tourney_id');
            location.href="/registerTeam?tourney_id="+tournament_id;
        }
    });
    $('#make-bracket').click(function(){
        var teams = $('.team');
        if(teams.length < 1){
            alert("Must have 2 teams to make a tournament");
        }else{
            alert("Add post to make  bracket");
        }
    })
});