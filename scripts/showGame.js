$(document).ready(function(){
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
        }
    });
    $('#reset-counts, #hit, #out').click(function(){
        clearCounts();
    });
    $('.base').click(function(e){
        var status = $(e.target).attr('data-status');
        if(status=='empty'){
            $(e.target).addClass('base-full');
            $(e.target).attr('data-status', 'full');
        }else{
            $(e.target).removeClass('base-full');
            $(e.target).attr('data-status', 'empty');
        }
    });    
});
function clearCounts(){
    $('.strike, .ball').css('background-color','black');
    $('#strikes').attr('data-count', 0);
    $('#balls').attr('data-count', 0);
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
            break;
        default:
            console.log('something went wrong');
            break;
    }
    if(!flag){
        $('#outs').attr('data-count', count + 1);
    }else{
        $('#outs').attr('data-count', 0);
    }
    clearCounts();
}
function switchInning(){
    var state = $('#inning').attr('data-status');
    var inning = parseInt($('#inning').html().split(' ')[1]);
    if(state=='top'){
        $('#inning').attr('data-status', 'bot');
        $('#inning').html('Bot ' + inning);
    }else{
        $('#inning').attr('data-status', 'top');
        $('#inning').html('Top ' + (inning + 1));
    }
}