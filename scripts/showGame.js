$(document).ready(function(){
    initItems();
    if(isNotWorker()){
        $('#controls').hide();
        $('#reset-counts').hide();
        $('#edit-score').hide();
    }
    $('#strike').click(function(){
        var count = parseInt($('#strikes').attr('data-count'));
        var flag = false;
        switch(count){
            case 0:
                $('#s1').css('background-color','red');
                break;
            case 1:
                $('#s2').css('background-color','red');
                break;
            case 2:
                $('#s3').css('background-color','red');
                flag = true;
                addOut();
                break;
            default:
                console.log('something went wrong');
                break;
        }
        if(!flag){
            $('#strikes').attr('data-count',count + 1);
            sendPost('strike', count + 1);
        }
    });
    $('#ball').click(function(){
        var count = parseInt($('#balls').attr('data-count'));
        var flag = false;
        switch(count){
            case 0:
                $('#b1').css('background-color','blue');
                break;
            case 1:
                $('#b2').css('background-color','blue');
                break;
            case 2:
                $('#b3').css('background-color','blue');
                break;
            case 3:
                $('#b4').css('background-color','blue');
                flag = true;
                clearCounts();
                break;
            default:
                console.log('something went wrong');
                break;
        }
        if(!flag){
            $('#balls').attr('data-count',count + 1);
            sendPost('ball', count + 1);
        }
    });
    $('#reset-counts').click(function(){
        clearCounts();
        $('.out').css('background-color','black');
        $('#outs').attr('data-count', 0);
        sendPost('out', 0);
    });
    $('#hit, #out').click(function(){
        clearCounts();
    });
    $('.base').click(function(e){
        const status = $(e.target).attr('data-status');
        if(status=='empty'){
            $(e.target).addClass('base-full');
            $(e.target).attr('data-status', 'full');
        }else{
            $(e.target).removeClass('base-full');
            $(e.target).attr('data-status', 'empty');
        }
    });
    $('#run').click(function(){
        const status = $('#inning').attr('data-status');
        const score1 = parseInt($('#team-score-1').html());
        const score2 = parseInt($('#team-score-2').html());
        if(status == 'top'){
            $('#team-score-1').html(score1 + 1);
            sendPost('score', (score1 + 1) + '-' + score2);
        }else{
            $('#team-score-2').html(score2 + 1);
            sendPost('score', score1 + '-' + (score2 + 1));
        }
    });
    $('#out').click(function(){
        addOut();
    })
    $('#to-tournament').click(function(ev){
        if(isNotWorker){
            location.href="/showTournament?tourney_id="+$(ev.target).attr('data-tourney-id');
        }else{
            location.href="/showTournament?tourney_id="+$(ev.target).attr('data-tourney-id') + "&worker=true";
        }
    });
    $('#edit-score').click(function(){
        const newScore = prompt('Enter new score here (format as "X-X" with team 1\'s score first');
        if(newScore){
            const newScores = newScore.split("-");
            $('#team-score-1').html(newScores[0]);
            $('#team-score-2').html(newScores[1]);
            sendPost('score', newScore);    
        }
    });
});
function clearCounts(){
    $('.strike, .ball').css('background-color','black');
    $('#strikes').attr('data-count', 0);
    sendPost('strike', 0);
    $('#balls').attr('data-count', 0);
    sendPost('ball', 0);
}
function addOut(){
    var count = parseInt($('#outs').attr('data-count'));
    var flag = false;
    switch(count){
        case 0:
            $('#o1').css('background-color','white');
            break;
        case 1:
            $('#o2').css('background-color','white');
            break;
        case 2:
            $('#o3').css('background-color','white');
            flag = true;
            switchInning();
            $('.out').css('background-color','black');
            sendPost('out', 0);
            break;
        default:
            console.log('something went wrong');
            break;
    }
    if(!flag){
        $('#outs').attr('data-count', count + 1);
        sendPost('out', count + 1);
    }else{
        $('#outs').attr('data-count', 0);
        sendPost('strike', 0);
    }
    clearCounts();
}
function switchInning(){
    var state = $('#inning').attr('data-status');
    var inning = parseInt($('#inning').html().split(' ')[1]);
    if(state=='top'){
        $('#inning').attr('data-status', 'bot');
        $('#inning').attr('data-batting', 2);
        $('#inning').html('Bot ' + inning);
        sendPost('team_batting', 2);
    }else{
        sendPost('team_batting', 1);
        const score1 = parseInt($('#team-score-1').html());
        const score2 = parseInt($('#team-score-2').html());
        if(Math.abs(score1-score2) >= 10 || inning > 2){
            if(score1!=score2){
                const name1 = $('#team-name-1').html();
                const name2 = $('#team-name-2').html();
                if(score1>score2){
                    $('#inning').html(name1 + ' wins after ' + inning + ' innings');
                }else{
                    $('#inning').html(name2 + ' wins after ' + inning + ' innings');
                }
                //remove event listeners
                finishGame();
            }else{
                $('#inning').attr('data-status', 'top');
                $('#inning').attr('data-batting', 2);
                $('#inning').html('Top ' + (inning + 1));        
                sendPost('inning', inning + 1);
            }
        }else{
            $('#inning').attr('data-status', 'top');
            $('#inning').attr('data-batting', 2);
            $('#inning').html('Top ' + (inning + 1));    
            sendPost('inning', inning + 1)
        }
    }
}
function sendPost(type, method){
    const tourney_id = getParameterByName('tourney_id');
    const game_id = getParameterByName('game_id');
    $.post('/updateDB?tourney_id=' + tourney_id + '&game_id=' + game_id + "&to_update=" + type + "&how_to_change=" + method, 
    function(data){
        data = JSON.parse(data);
        console.log(data);
        if(!data.success){
            alert('Something went wrong\nNeed to refresh');
            location.reload();
        };
    }).fail(function(data){
        data = JSON.parse(data);
        console.log(data);
        alert('Something went wrong\nNeed to refresh');
        location.reload();
    });
}
function initItems(){
    const scores = $('#scores').attr('data-score').split('-');
    $('#team-score-1').html(scores[0]);
    $('#team-score-2').html(scores[1]);
    const batting = $('#inning').attr('data-batting');
    const inning = $('#inning').html();
    if(batting != -1){
        if(batting == 1){
            $('#inning').attr('data-status','top');
            $('#inning').html('Top ' + inning);
        }else{
            $('#inning').attr('data-status','bot');
            $('#inning').html('Bot ' + inning);
        }
    }else{
        if( scores[0] > scores[1]){
            const name1 = $('#team-name-1').html();
            $('#inning').html(name1 + ' wins after ' + inning + ' innings');
        }else{
            const name2 = $('#team-name-2').html();
            $('#inning').html(name2 + ' wins after ' + inning + ' innings');

        }
        $('#controls, #reset-counts, #edit-score').hide();
    }
    const strikes = parseInt($('#strikes').attr('data-count'));
    switch(strikes){
        case 1:
            $('#s1').css('background-color','red');
            break;
        case 2:
            $('#s1').css('background-color','red');
            $('#s2').css('background-color','red');
    }
    const balls = parseInt($('#balls').attr('data-count'));
    switch(balls){
        case 1:
            $('#b1').css('background-color','blue');
            break;
        case 2:
            $('#b1').css('background-color','blue');
            $('#b2').css('background-color','blue');
    }
    const outs = parseInt($('#outs').attr('data-count'));
    switch(outs){
        case 1:
            $('#o1').css('background-color','white');
            break;
        case 2:
            $('#o1').css('background-color','white');
            $('#o2').css('background-color','white');
    }
}
function finishGame(){
    $('#controls, #reset-counts, #edit-score').hide();
    sendPost('team_batting', -1);
    const level = $('#game-level').html();
    if(level!='Final'){
        const tourney_id = getParameterByName('tourney_id');
        const game_id = getParameterByName('game_id');
        $.post('/gameFinsihed?tourney_id=' + tourney_id + '&game_id=' + game_id, 
        function(data){
            data = JSON.parse(data);
            console.log(data);
            if(!data.success){
                alert('Something went wrong\nNeed to refresh');
                location.reload();
            };
        }).fail(function(data){
            data = JSON.parse(data);
            console.log(data);
            alert('Something went wrong\nNeed to refresh');
            location.reload();
        });    
    }
}