$(document).ready(function(){
    const teams = $('.team');
    const games = $('.game');
    if(teams.length >= 4){
        $('#register-team').hide();
    }else if(teams.length > 2){
        $('#make-bracket').show();
    }else{
        $('#make-bracket').hide();
    }
    if(games.length > 0){
        $('#make-bracket').hide();
        $('#register-team').hide();
    }
    $('#register-team').click(function(){
        //get query parm
        var tournament_id = getParameterByName('tourney_id');
        location.href="/registerTeam?tourney_id="+tournament_id;
    });
    $('#make-bracket').click(function(){
        var tournament_id = getParameterByName('tourney_id');
        $.post('/registerGames?tourney_id='+tournament_id,function(){
            location.reload();
        });
    })
});